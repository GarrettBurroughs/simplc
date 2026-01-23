use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
    process::exit,
};

use env_logger::Builder;
use inkwell::context::Context;
use log::{debug, info};
use miette::NamedSource;

use crate::{
    error::CompilerError,
    frontend::{
        ast_visualizer::ASTVisualizer, lexer::Lexer, parser::Parser,
        source_ast_visualizer::SourceASTVisualizer,
    },
    semantic::{label_resolution::resolve_labels, variable_resolution::resolve_variables},
    sourcemap::SourceFile,
};

mod codegen;
mod error;
mod frontend;
mod semantic;
mod sourcemap;

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

    /// Output AST with source
    #[arg(long)]
    source_ast: bool,

    /// Debug level
    #[arg(long)]
    debug_level: Option<log::LevelFilter>,
}

fn main() {
    let args = <Args as clap::Parser>::parse();

    Builder::new()
        .format_line_number(true)
        .format_file(true)
        .filter_level(args.debug_level.unwrap_or(log::LevelFilter::Off))
        .init();

    let binding = if let Some(out) = &args.output {
        PathBuf::from(out)
    } else {
        let mut path = PathBuf::from(&args.file);
        path.set_extension("");
        path
    };

    let output_name = binding.to_str().unwrap_or("out");

    info!("Starting Compilation for {}", output_name);

    let contents = match fs::read_to_string(&args.file) {
        Ok(c) => c,
        Err(_) => exit(1),
    };

    let source_file = SourceFile::new(&args.file, contents);

    if let Err(e) = run(&source_file, output_name, args) {
        let report = miette::Error::from(e).with_source_code(NamedSource::new(
            source_file.file_path,
            source_file.contents.clone(),
        ));
        // let error_message = match e {
        //     CompilerError::LexError {
        //         location,
        //         character,
        //     } => {
        //         format!(
        //             "Parse Error [{}]: unexpected character {}",
        //             source_file.display(location.start),
        //             character
        //         )
        //     }
        //     CompilerError::ParseError { location, kind } => {
        //         format!(
        //             "Parse Error [{}]: {}",
        //             source_file.display(location.start),
        //             kind
        //         )
        //     }
        //     CompilerError::SemanticError { location, kind } => {
        //         format!(
        //             "Semantic Error [{}]: {}",
        //             source_file.display(location.start),
        //             kind
        //         )
        //     }
        //     CompilerError::SystemError { kind } => format!("System Error: {}", kind),
        // };
        eprintln!("{:?}", report);
        exit(1);
    };
}

fn write_error(file: &str) -> CompilerError {
    CompilerError::SystemError {
        kind: error::SystemErrorKind::FileWrite(file.to_string()),
    }
}

fn run(source_file: &SourceFile, output_name: &str, args: Args) -> Result<(), CompilerError> {
    info!("Running Lexical Analysis for {}", output_name);
    // Lexical Analysis
    let lexer = Lexer::new(&source_file);
    let tokens: Vec<_> = lexer.collect::<Result<Vec<_>, _>>()?;
    tokens.iter().for_each(|t| debug!("{}", t));

    if args.lex {
        let path = format!("{}.lex", output_name);
        let mut file = File::create(&path).map_err(|_| write_error(&path))?;
        for tok in &tokens {
            writeln!(file, "{}", tok).map_err(|_| write_error(&path))?;
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

    let ast_visualizer = ASTVisualizer::new(&source_file);
    let ast_viz = ast_visualizer.visualize(&program);
    debug!("AST: \n{}", ast_viz);

    if args.ast {
        let path = format!("{}.ast", output_name);
        let mut file = File::create(&path).map_err(|_| write_error(&path))?;
        writeln!(file, "{}", ast_viz).map_err(|_| write_error(&path))?;
    }

    let source_visualizer = SourceASTVisualizer::new(&source_file);
    let source_viz = source_visualizer.visualize(&program);
    debug!("Source Visualizer:\n {}", source_viz);
    if args.source_ast {
        let path = format!("{}.srcmp", output_name);
        let mut file = File::create(&path).map_err(|_| write_error(&path))?;
        writeln!(file, "{}", source_viz).map_err(|_| write_error(&path))?;
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
        let mut file = File::create(&path).map_err(|_| write_error(&path))?;
        writeln!(file, "{}", ir).map_err(|_| write_error(&path))?;
    }

    let buf = generator
        .emit_assmebly()
        .map_err(|_| CompilerError::SystemError {
            kind: error::SystemErrorKind::AssemblyGeneration,
        })?;
    let path = format!("{}.s", output_name);
    let mut file = File::create(&path).map_err(|_| write_error(&path))?;
    file.write_all(buf.as_slice())
        .map_err(|_| write_error(&path))?;
    info!("Wrote generated assembly to: {}.s", output_name);

    Ok(())
}
