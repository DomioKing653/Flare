#[derive(Debug, Clone)]
pub enum Instructions {
    Add,
    Sub,
    Div,
    Mul,
    GreaterThan,
    LessThan,
    //Variables
    LoadVar(String),
    SaveVar(String),
    //Values
    PushString(String),
    PushBool(bool),
    PushNumber(f32),
    //Printing
    WriteLnLastOnStack,
    WriteLastOnStack,
    If(usize),
    //Process
    ProcessExit,
    //Control flow
    Jump(usize),
    JumpIfFalse(usize),

    ReadInput,
    // Halt
    Halt,
}
