use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
    process::exit,
};

use env_logger::Builder;
use inkwell::context::Context;
use log::{debug, info};

use crate::{
    frontend::{lexer::Lexer, parser::Parser},
    semantic::{label_resolution::resolve_labels, variable_resolution::resolve_variables},
};

mod codegen;
mod error;
mod frontend;
mod semantic;

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

    /// Output AST
    #[arg(long)]
    ast: bool,

    /// Debug level
    #[arg(long)]
    debug_level: Option<log::LevelFilter>
}

fn main() {
    let args = <Args as clap::Parser>::parse();

    Builder::new()
        .format_line_number(true)
        .format_file(true)
        .filter_level(args.debug_level.unwrap_or(log::LevelFilter::Off))
        .init();

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

    let output_name = binding.to_str().unwrap_or("out");

    info!("Starting Compilation for {}", output_name);

    let contents = fs::read_to_string(&args.file)
        .map_err(|_| format!("Could not read file: {}", args.file))?;

    info!("Running Lexical Analysis for {}", output_name);
    // Lexical Analysis
    let lexer = Lexer::new(&contents);
    let tokens: Vec<_> = lexer.collect::<Result<Vec<_>, _>>()?;
    tokens.iter().for_each(|t| debug!("{}", t));

    if args.lex {
        let path = format!("{}.lex", output_name);
        let mut file = File::create(&path)?;
        for tok in &tokens {
            writeln!(file, "{}", tok)?;
        }
    }

    info!("Parsing {}", output_name);
    // Parsing
    let mut parser = Parser::new(tokens);
    let mut program = parser.parse_program()?;

    // Semantic Passes
    info!("Running variable resolution for {}", output_name);
    let variables = resolve_variables(&mut program)?;
    debug!("Symbol Table: {:?}", variables);
    info!("Running label resolution for {}", output_name);
    let labels = resolve_labels(&mut program)?;
    debug!("Label Table: {:?}", labels);

    debug!("AST: \n{}", program.visualize());

    if args.ast {
        let path = format!("{}.ast", output_name);
        let mut file = File::create(&path)?;
        writeln!(file, "{}", program.visualize())?;
    }

    // Code Generation
    info!("Running code generation for {}", output_name);
    let context = Context::create();
    let mut generator = codegen::codegen::CodeGen::new(&context, "main", variables);
    generator.run_codegen(&program);

    let ir = generator.emit_ir();
    debug!("Intermediate Representation: \n {}", ir);

    if args.ir {
        let path = format!("{}.ll", output_name);
        let mut file = File::create(&path)?;
        writeln!(file, "{}", ir)?;
    }

    let buf = generator.emit_assmebly()?;
    let path = format!("{}.s", output_name);
    let mut file = File::create(&path)?;
    file.write_all(buf.as_slice())?;
    info!("Wrote generated assembly to: {}.s", output_name);

    Ok(())
}
