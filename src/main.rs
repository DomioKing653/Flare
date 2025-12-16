
//Mod
mod lexer;
mod errors;
mod ast;
//Imports
use std::fs;
use std::env;
use crate::ast::parser::Parser;
use crate::errors::lexer_errors::LexerError;
use std::process;
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
                    Ok(build(args[2].clone()))
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

fn build(dir: String){
    println!("Building {}", dir);
    //LEXER
    let mut main_lexer:Tokenizer=Tokenizer::new(fs::read_to_string(dir).unwrap());
    let tokens :&Vec<Token> = main_lexer.tokenize().unwrap_or_else(|e| {
        match e.error_type {
            UnknownTokenError => {
                println!("Unknown token:{}!", e.wrong_token);
                process::exit(-1);
            }
            MoreDotInANumberError => {
                println!("Cannot have more than one dot in a number:{}!", e.wrong_token);
                process::exit(-1);
            }
        }
    });
    for token in tokens {
        println!("{:?}",token);
    }
    //PARSER
    let mut main_parser:Parser = Parser::new(tokens.to_vec());
    let mut parsed_ast= main_parser.parse().unwrap_or_else(|e|{
        match e.error_type {
            crate::errors::parser_errors::ParserErrorType::UnexpectedTokenAtFactor=>{
                println!("Unexpected token->expected value but found:{:?}",e.wrong_token.token_kind);
                process::exit(-2);
            }
        }
    });
    println!("{:?}",parsed_ast);
    //INTERPRETER
    println!("{:?}",parsed_ast.visit_node());
}