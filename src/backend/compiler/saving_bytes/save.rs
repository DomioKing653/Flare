use crate::backend::{
    ast::parser::Parser,
    compiler::{
        byte_code::{Compilable, Compiler},
        instructions::Instructions,
    },
    lexer::{tokenizer::Tokenizer, tokens::Token},
};

use std::{
    fs,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
    process,
    time::Instant,
};

fn debug_print(tokens: &Vec<Token>, ast: Box<dyn Compilable>, instructions: &Vec<Instructions>) {
    for token in tokens {
        println!("{:?}", token);
    }
    println!("{:?}", ast);
    for instruction in instructions {
        println!("{:?}", instruction);
    }
}

pub fn build(dir: String, out: String, debug: bool) {
    ensure_target_dir();

    // Start timing
    let start_time = Instant::now();

    // Get the absolute path for display
    let src_path = std::path::Path::new(&dir)
        .canonicalize()
        .unwrap_or_else(|_| std::path::PathBuf::from(&dir));

    println!(
        "\x1b[1;32mBuilding\x1b[0m {} -> target/{}",
        src_path.display(),
        out
    );

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
    /*
     * Parser
     */
    let mut main_parser: Parser = Parser::new(tokens.to_vec());
    let parsed_ast = main_parser.parse().unwrap_or_else(|e| {
        println!("\x1b[1;31m{}\x1b[0m",e);
        process::exit(-2)
    });
    /*
     *Bytecode
     */
    let mut compiler = Compiler::new();
    if let Err(e) = parsed_ast.compile(&mut compiler) {
        println!("\x1b[1;31m{}\x1b[0m", e);
        println!("\x1b[1mTry:flarec error <error code> for fix\x1b[0m");
        process::exit(-3);
    }
    compiler.optimize();

    // Print debug information if debug flag is enabled
    if debug {
        debug_print(tokens, parsed_ast, &compiler.out);
    }

    let out_path = format!("out/{}", out);
    compile_to_exec(out_path, &mut compiler.out).expect("Cannot load binary file");

    // Calculate elapsed time and show success message
    let elapsed = start_time.elapsed();
    let seconds = elapsed.as_secs_f32();

    println!("\x1b[1;32mFinished\x1b[0m in {:.3} seconds", seconds);
}

fn compile_to_exec(file_name: String, byte_code: &mut Vec<Instructions>) -> std::io::Result<()> {
    let file = File::create(file_name)?;
    let mut writer = BufWriter::new(file);
    for instr in byte_code {
        let opcode = instr.opcode();
        match instr {
            Instructions::Add => writer.write_all(&[opcode])?,
            Instructions::Sub => writer.write_all(&[opcode])?,
            Instructions::Mul => writer.write_all(&[opcode])?,
            Instructions::Div => writer.write_all(&[opcode])?, 
            Instructions::Modulo => writer.write_all(&[opcode])?,
            Instructions::PushString(s) => {
                writer.write_all(&[opcode])?;
                let bytes = s.as_bytes();
                writer.write_all(&(bytes.len() as u32).to_le_bytes())?;
                writer.write_all(&s.as_bytes())?
            }

            //Values
            Instructions::PushBool(b) => {
                writer.write_all(&[opcode])?;
                writer.write_all(&[*b as u8])?;
            }
            Instructions::PushNumber(n) => {
                writer.write_all(&[opcode])?;
                writer.write_all(&n.to_le_bytes())?;
            }

            Instructions::WriteLnLastOnStack => {
                writer.write_all(&[opcode])?;
            }
            Instructions::WriteLastOnStack => {
                writer.write_all(&[opcode])?;
            }
            Instructions::ProcessExit => {
                writer.write_all(&[opcode])?;
            }
            Instructions::JumpIfTrue(adr) => {
                writer.write_all(&[opcode])?;
                writer.write_all(&(*adr as u16).to_le_bytes())?;
            }
           
            Instructions::LoadVar(v) => {
                writer.write_all(&[opcode])?;
                let bytes = v.as_bytes();
                writer.write_all(&(bytes.len() as u32).to_le_bytes())?;
                writer.write_all(&v.as_bytes())?
            }
            Instructions::SaveVar(v) => {
                writer.write_all(&[opcode])?;
                let bytes = v.as_bytes();
                writer.write_all(&(bytes.len() as u32).to_le_bytes())?;
                writer.write_all(&v.as_bytes())?
            }
            Instructions::Jump(adr) => {
                writer.write_all(&[opcode])?;
                writer.write_all(&(*adr as u16).to_le_bytes())?;
            }
            Instructions::JumpIfFalse(adr) => {
                writer.write_all(&[opcode])?;
                writer.write_all(&(*adr as u16).to_le_bytes())?;
            }
            Instructions::GreaterThan => {
                writer.write_all(&[opcode])?;
            }
            Instructions::LessThan => {
                writer.write_all(&[opcode])?;
            }
            Instructions::Equal => {
                writer.write_all(&[opcode])?;
            }
            Instructions::ReadInput => {
                writer.write_all(&[opcode])?;
            }

            Instructions::Halt => writer.write_all(&[opcode])?,
        }
    }
    Ok(())
}

fn ensure_target_dir() {
    let target = Path::new("out");
    if !target.exists() {
        fs::create_dir(target).expect("Cannot create target directory");
    }
}
