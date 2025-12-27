use crate::{
    ast::parser::Parser,
    compiler::{byte_code::Compiler, instructions::Instructions},
    errors::parser_errors::ParserError,
    lexer::{tokenizer::Tokenizer, tokens::Token},
    virtual_machine::virtual_machine::VM,
};
use std::{
    fs,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
    process,
};

pub fn build(dir: String, out: String) {
    ensure_target_dir();
    println!("Building {}", dir);
    /*
     * Lexer
     */
    let mut main_lexer: Tokenizer = Tokenizer::new(fs::read_to_string(dir).unwrap());
    let tokens: &Vec<Token> = match main_lexer.tokenize() {
        Err(e) => {
            print!("{}", e);
            process::exit(-1);
        }
        Ok(tokens) => tokens,
    };
    for token in tokens {
        println!("{:?}", token);
    }
    /*
     * Parser
     */
    let mut main_parser: Parser = Parser::new(tokens.to_vec());
    let parsed_ast = main_parser.parse().unwrap_or_else(|e| {
        match e {
            ParserError::UnexpectedToken { expected, found } => {
                println!("Expected: {:?} but found {:?}", expected, found)
            }
        }
        process::exit(-2)
    });
    println!("\n{:?}", parsed_ast);
    /*
     *Bytecode
     */
    let mut compiler = Compiler::new();
    if let Err(e) = parsed_ast.compile(&mut compiler) {
        println!("{}", e);
        process::exit(-3);
    }
    compiler.optimize();
    for instruction in compiler.out.iter().clone() {
        println!("{:?}", instruction);
    }
    let out_path = format!("target/{}", out);
    compile_to_exec(out_path, &mut compiler.out).expect("Cannot load binary file");
}

fn compile_to_exec(file_name: String, byte_code: &mut Vec<Instructions>) -> std::io::Result<()> {
    let file = File::create(file_name)?;
    let mut writer = BufWriter::new(file);
    for instr in byte_code {
        match instr {
            Instructions::Add => writer.write_all(&[1u8])?,
            Instructions::Sub => writer.write_all(&[2u8])?,

            Instructions::Mul => writer.write_all(&[3u8])?,
            Instructions::Div => writer.write_all(&[4u8])?,
            Instructions::PushString(s) => {
                writer.write_all(&[5u8])?;
                let bytes = s.as_bytes();
                writer.write_all(&(bytes.len() as u32).to_le_bytes())?;
                writer.write_all(&s.as_bytes())?
            }
            Instructions::LoadVar(v) => {
                writer.write_all(&[6u8])?;
                let bytes = v.as_bytes();
                writer.write_all(&(bytes.len() as u32).to_le_bytes())?;
                writer.write_all(&v.as_bytes())?
            }
            Instructions::SaveVar(v) => {
                writer.write_all(&[7u8])?;
                let bytes = v.as_bytes();
                writer.write_all(&(bytes.len() as u32).to_le_bytes())?;
                writer.write_all(&v.as_bytes())?
            }
            Instructions::PushBool(b) => {
                writer.write_all(&[8u8])?;
                writer.write_all(&[*b as u8])?;
            }
            Instructions::PushNumber(n) => {
                writer.write_all(&[9u8])?; // opcode pro PushNumber
                writer.write_all(&n.to_le_bytes())?;
            }
            Instructions::WriteLnLastOnStack => {
                writer.write_all(&[20u8])?;
            }
            Instructions::WriteLastOnStack => {
                writer.write_all(&[21u8])?;
            }
            Instructions::If(instruction_count) => {
                writer.write_all(&[30])?;
                writer.write_all(&(*instruction_count as u8).to_le_bytes())?;
            }
            Instructions::ProcessExit => {
                writer.write_all(&[35u8])?;
            }
            Instructions::Halt => writer.write_all(&[255u8])?,
        }
    }
    Ok(())
}

pub fn run_code(path: &str) {
    let mut vm: VM = VM::from_file(path).unwrap();
    println!("Program:");
    vm.run().unwrap()
}
fn ensure_target_dir() {
    let target = Path::new("target");
    if !target.exists() {
        fs::create_dir(target).expect("Cannot create target directory");
    }
}
