//Imports
use std::env;
use flare::{
    compiler::saving_bytes::save::{build, run_code},
    errors::cli_errors::CommandLineError,
    errors::cli_errors::CommandLineError::{BuildHasJustTwoArg, NoFileSpecifiedForBuild, NoSuchCommand},
};

fn main() {
    if let Err(e) = run_cli(){
        eprintln!("Fatal error:{:?}!",match e{
            BuildHasJustTwoArg =>"Build command has just two arguments",
            NoFileSpecifiedForBuild=>"No file specified for build",
            NoSuchCommand=>"No such command",
        }.to_string());
    }
}

fn run_cli() -> Result<(), CommandLineError>{
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(NoSuchCommand);
    }

    match args[1].as_str() {
        "build" => {
            if args.len() != 4 {
                return Err(BuildHasJustTwoArg);
            }
            build(args[2].clone(), args[3].clone());
            Ok(())
        }
        "run" => {
            if args.len() != 3 {
                return Err(NoFileSpecifiedForBuild);
            }
            run_code(&args[2].clone());
            Ok(())
        }
        "exec" => {
            if args.len() != 4 {
                return Err(BuildHasJustTwoArg);
            }
            build(args[2].clone(), args[3].clone());
            run_code(&format!("target/{}",&args[3].clone()));
            Ok(())
        }
        _ => Err(NoSuchCommand),
    }
}