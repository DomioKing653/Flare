
//Mod
mod lexer;
mod errors;
mod ast;
mod compiler;

//Imports
use std::fs;
use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use crate::ast::parser::Parser;
use std::process;
use crate::compiler::instructions::Instructions;
use crate::errors::lexer_errors::LexerErrorType::{MoreDotInANumberError, UnknownTokenError};
use crate::lexer::tokenizer::Tokenizer;
use crate::lexer::tokens::Token;
use crate::errors::cli_errors::CommandLineError;
use crate::errors::cli_errors::CommandLineError::{BuildHasJustTwoArg, NoFileSpecifiedForBuild, NoSuchCommand};

fn main() {
    if let Err(e) = run_cli(){
        eprintln!("Fatal error:{:?}!",match e{
            BuildHasJustTwoArg =>"Build command has just two arguments",
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
                if args.len() <= 3 {
                    Err(NoFileSpecifiedForBuild)
                } else if args.len() > 4 {
                    Err(BuildHasJustTwoArg)
                } else {
                    Ok(build(args[2].clone(),args[3].clone()))
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

fn build(dir: String, out:String){
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
    let parsed_ast= main_parser.parse().unwrap_or_else(|e|{
        match e.error_type {
            errors::parser_errors::ParserErrorType::UnexpectedTokenAtFactor=>{
                println!("Unexpected token->expected value but found:{:?}",e.wrong_token.token_kind);
                process::exit(-2);
            }
        }
    });
    println!("{:?}",parsed_ast);
    //VM
    let byte_code:&mut Vec<Instructions> = &mut vec![];
    parsed_ast.compile(byte_code);
    println!("{:?}",byte_code);
    compile_to_exec(out, byte_code).expect("TODO: panic message");
}

fn compile_to_exec(file_name:String,byte_code:&mut Vec<Instructions>)->std::io::Result<()>{
    let file = File::create(file_name)?;
    let mut writer = BufWriter::new(file);
    for instr in byte_code {
        match instr {
            Instructions::PushNumber(n) => {
                writer.write_all(&[0u8])?; // opcode pro PushNumber
                writer.write_all(&n.to_le_bytes())?;
            }
            Instructions::Add => writer.write_all(&[1u8])?,
            Instructions::Sub => writer.write_all(&[2u8])?,
            Instructions::Mul => writer.write_all(&[3u8])?,
            Instructions::Div => writer.write_all(&[4u8])?,
            Instructions::Halt =>{ writer.write_all(&[255u8])?;println!("Halt")},
        }
    }
    Ok(())
}