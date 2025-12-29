use std::{fs::{self, File}, io::Write, process::exit};

use clap::Parser;
use thiserror::Error;

use crate::frontend::lexer::{Lexer, TokenLocation};

mod frontend;

#[derive(Error, Debug, PartialEq, Eq)]
enum CompilerError {
    #[error("Lex Error at {0} {1}")]
    LexError(usize, usize),
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

    let file_contents = fs::read_to_string(&args.file);
    match file_contents {
        Ok(file_contents) => {
            let mut l = Lexer::new(file_contents.trim().to_string());
            let mut tokens: Vec<TokenLocation> = Vec::new();
            while l.has_next_token() {
                match l.get_next_token() {
                    Ok(tok) => {
                        tokens.push(tok);
                    }
                    Err(err) => match err {
                        CompilerError::LexError(row, col) => {
                            eprintln!("Lex error at {} {}", row, col)
                        }
                    },
                }
            }
            if args.lex {
                if let Ok(mut output_file) = File::create(args.output.clone() + ".lex") {
                    for tok in tokens {
                        if args.print {
                            println!("{}", tok);
                        }
                        output_file.write_all(tok.to_string().as_bytes()).unwrap();
                        output_file.write_all("\n".as_bytes()).unwrap();
                    }
                } else {
                    eprintln!("Cannot write to {}", args.output.clone() + ".lex");
                    exit(1);
                }
            }
        }
        Err(_) => {
            eprintln!("Cannot open file: {}", &args.file)
        }
    }
}
