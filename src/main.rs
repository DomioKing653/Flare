
//Mod
mod lexer;
mod errors;
mod ast;

//Imports
use std::fs;
use std::env;
use crate::ast::parser::Parser;
use crate::errors::lexer_errors::LexerError;
use crate::errors::lexer_errors::LexerErrorType::{MoreDotInANumberError, UnknownTokenError};
use crate::lexer::tokenizer::Tokenizer;
use crate::lexer::tokens::Token;
use crate::errors::cli_errors::CommandLineError;
use crate::errors::cli_errors::CommandLineError::{BuildHasJustOneArg, NoFileSpecifiedForBuild, NoSuchCommand};

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
    //LEXER
    let mut main_lexer:Tokenizer=Tokenizer::new(fs::read_to_string(dir).unwrap());
    let tokens :&Vec<Token> = main_lexer.tokenize()?;
    for token in tokens {
        println!("{:?}",token);
    }
    //PARSER
    let mut mainParser:Parser = Parser::new(tokens.to_vec());
    Ok(())
}