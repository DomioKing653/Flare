mod lexer;
use std::fs;
use std::env;
use std::fmt::Debug;
use crate::CommandLineError::{BuildHasJustOneArg, NoFileSpecifiedForBuild, NoSuchCommand};
use crate::lexer::tokenizer::{LexerError, Tokenizer};
use crate::lexer::tokens::Token;
use crate::lexer::tokenizer::LexerErrorType::UnknownTokenError;
use crate::lexer::tokenizer::LexerErrorType::MoreDotInANumberError;

#[derive(Debug)]
enum CommandLineError{
    BuildHasJustOneArg,
    NoFileSpecifiedForBuild,
    NoSuchCommand
}
fn main() {
    if let Err(e) = run_cli(){
        eprintln!("Fatal error:{:?}!",match e{
            BuildHasJustOneArg=>"Build command has just one argument",
            NoFileSpecifiedForBuild=>"No file specified for build",
            NoSuchCommand=>"No such command",
        }.to_string());
    }
}

fn run_cli() ->Result<(),CommandLineError> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        return match args[1].as_str() {
            "build" => {
                if args.len() <= 2 {
                    Err(NoFileSpecifiedForBuild)
                } else if args.len() > 3 {
                    Err(BuildHasJustOneArg)
                } else {
                    if let Err(e) =  build(&args[2]){
                        match e.error_type {
                            UnknownTokenError=>{
                                println!("Unknown token:{}!",e.wrong_token);
                            }
                            MoreDotInANumberError=>{
                                println!("Cannot have more than one dot in a number:{}!",e.wrong_token)
                            }
                        }
                    }
                    Ok(())
                }
            }
            "console" => {
                Ok(())
            }
            _ => Err(NoSuchCommand),
        }
    }
    Ok(())
}

fn build(dir: &str)->Result<(),LexerError> {
    println!("Building {}", dir);
    let mut main_lexer:Tokenizer=Tokenizer::new(fs::read_to_string(dir).unwrap());
    let tokens :&Vec<Token> = main_lexer.tokenize()?;
    for token in tokens {
        println!("{:?}",token);
    }

    Ok(())
}