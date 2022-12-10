mod pos;
mod lexer;
mod ast;
lalrpop_mod!(parser);

use clap::Parser;
use rustyline::error::ReadlineError;
use rustyline;
use lalrpop_util::lalrpop_mod;

use lexer::Lexer;
use parser::ExprParser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
struct Args {
    #[arg(long)]
    debug: bool
}

const PROMPT: &'static str = "fvm> ";
const HISTORY_FILENAME: &'static str = ".fvm-history.txt";
    
fn main() {
    let Args {debug} = Args::parse();

    let mut rl = rustyline::Editor::<()>::new().unwrap();

    if rl.load_history(HISTORY_FILENAME).is_err() {
        println!("No previous history.");
    }

    loop {
        match rl.readline(PROMPT) {
            Ok(line) => {
                rl.add_history_entry(line.as_str());

                if debug {
                    println!("Tokens\n======\n");

                    for res in Lexer::new(line.as_str(), None) {
                        match res {
                            Ok((start, tok, end)) => {
                                print!("<{:?} in ", tok);

                                match start.filename {
                                    Some(filename) => print!("{}", filename),
                                    None => print!("<unknown>")
                                }

                                println!(" at {}:{}-{}:{}>", start.line, start.col, end.line, end.col);
                            },

                            Err(err) => {
                                eprintln!("Lexical error: {}", err);
                                break;
                            }
                        }
                    }

                    println!("\nAST\n===\n");
                }

                match ExprParser::new().parse(Lexer::new(line.as_str(), None)) {
                    Ok(expr) => if debug { println!("{}", expr); },

                    Err(err) => {
                        eprintln!("Parse error: {}", err);
                        continue
                    }
                };
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    rl.save_history(HISTORY_FILENAME).unwrap();
}