use std::{
    fs::{self, File}, io::Write, path::{PathBuf}, process::exit
};

use inkwell::context::Context;

use crate::{frontend::{lexer::Lexer, parser::Parser}, semantic::{label_resolution::resolve_labels, variable_resolution::resolve_variables}};

mod frontend;
mod codegen;
mod semantic;
mod error;


#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// File to compile
    file: String,

    /// Only output lexical analyis
    #[arg(short, long)]
    lex: bool,

    /// Output file location
    #[arg(short, long)]
    output: Option<String>,

    /// Print output to stdout
    #[arg(short, long)]
    print: bool,

    /// Output LLVM Ir
    #[arg(long)]
    ir: bool,

    /// Output assembly
    #[arg(long)]
    asm: bool,

    // Output AST
    #[arg(long)]
    ast: bool,
}

fn main() {
    let args = <Args as clap::Parser>::parse();
    if let Err(e) = run(args) {
        eprintln!("{}", e);
        exit(1);
    };
}

fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    let binding = if let Some(out) = args.output {
        PathBuf::from(out)
    } else {
        let mut path = PathBuf::from(&args.file);
        path.set_extension(""); 
        path
    };

    let output_name = 
        binding.to_str().unwrap_or("out");

    let contents = fs::read_to_string(&args.file)
        .map_err(|_| format!("Could not read file: {}", args.file))?;

    // Lexical Analysis
    let lexer = Lexer::new(&contents);
    let tokens: Vec<_> = lexer.collect::<Result<Vec<_>, _>>()?;

    if args.lex {
        let path = format!("{}.lex", output_name);
        let mut file = File::create(&path)?;
        for tok in &tokens {
            if args.print {
                println!("{}", tok);
            }
            writeln!(file, "{}", tok)?;
        }
    }

    // Parsing
    let mut parser = Parser::new(tokens);
    let mut program = parser.parse_program()?;

    // Semantic Passes
    resolve_variables(&mut program)?;
    resolve_labels(&mut program)?;

    if args.ast {
        if args.print {
            println!("{}", program.visualize());
        }

        let path = format!("{}.ast", output_name);
        let mut file = File::create(&path)?;
        writeln!(file, "{}", program.visualize())?;
    }


    // Code Generation
    let context = Context::create();
    let mut generator = codegen::codegen::CodeGen::new(&context, "main");
    generator.run_codegen(&program);

    if args.ir {
        let ir = generator.emit_ir();
        if args.print {
            println!("Intermediate Representation: \n {}", ir);
        }
        let path = format!("{}.ll", output_name);
        let mut file = File::create(&path)?;
        writeln!(file, "{}", ir)?;
    }

    
    let buf = generator.emit_assmebly()?;
    let path = format!("{}.s", output_name);
    let mut file = File::create(&path)?;
    file.write_all(buf.as_slice())?;

    Ok(())
}
