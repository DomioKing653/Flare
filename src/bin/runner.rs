use std::fs;
use std::process::Command;

fn main() {
    let cfg = fs::read_to_string("run.cfg")
        .expect("Missing run.cfg");

    let mut bytecode = "";

    for line in cfg.lines() {
        if let Some(v) = line.strip_prefix("bytecode=") {
            bytecode = v;
        }
    }

    if bytecode.is_empty() {
        panic!("No bytecode in run.cfg");
    }

    Command::new("flare")
        .args(["run", bytecode])
        .status()
        .expect("Failed to run flare");
}