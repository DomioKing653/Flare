#[derive(Debug)]
pub enum CommandLineError{
    BuildHasJustOneArg,
    NoFileSpecifiedForBuild,
    NoSuchCommand
}