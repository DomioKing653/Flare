use flare::runtime::virtual_machine::virtual_machine::VM;

static PROGRAM: &[u8] = include_bytes!("../../out/program.bin");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut vm = VM::from_bytes(PROGRAM.to_vec())?;
    vm.run()?;
    Ok(())
}
