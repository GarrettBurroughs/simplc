use std::{
    fs::{self, File},
    io::Write,
    process::exit,
};

use clap::Parser;
use thiserror::Error;

use crate::frontend::{ast::AST, lexer::Lexer};

mod frontend;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum CompilerError {
    #[error("Lex Error at {0} {1}")]
    LexError(usize, usize),

    #[error("Parse Error at {0} {1} {2}")]
    ParseError(usize, usize, String),
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// File to compile
    file: String,

    /// Only output lexical analyis
    #[arg(short, long)]
    lex: bool,

    /// Output file location
    #[arg(short, long, default_value = "out")]
    output: String,

    /// Print output to stdout
    #[arg(short, long)]
    print: bool,
}

fn main() {
    let args = Args::parse();
    if let Err(e) = run(args) {
        eprintln!("{}", e);
        exit(1);
    };
}

fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(&args.file)
        .map_err(|_| format!("Could not read file: {}", args.file))?;

    let lexer = Lexer::new(&contents);
    let tokens: Vec<_> = lexer.collect::<Result<Vec<_>, _>>()?;

    if args.lex {
        let path = format!("{}.lex", args.output);
        let mut file = File::create(&path)?;
        for tok in &tokens {
            if args.print {
                println!("{}", tok);
            }
            writeln!(file, "{}", tok)?;
        }
    }

    let mut ast = AST::new(tokens);
    let program = ast.parse_program()?;
    println!("{:#?}", program);

    Ok(())
}
