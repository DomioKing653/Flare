#[derive(Debug)]
pub enum CommandLineError{
    BuildHasJustTwoArg,
    NoFileSpecifiedForBuild,
    NoSuchCommand
}