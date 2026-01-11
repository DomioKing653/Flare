#[derive(Debug, Clone)]
pub enum Instructions {
    Add,
    Sub,
    Div,
    Mul,
    Modulo,
    //Comparison
    GreaterThan,
    LessThan,
    Equal,
    //Variables
    LoadVar(String),
    SaveVar(String),
    //Values
    PushString(String),
    PushBool(bool),
    PushNumber(f32),
    PushFloat(f32),
    //Printing
    WriteLnLastOnStack,
    WriteLastOnStack,
    //Process
    ProcessExit,
    //Control flow
    Jump(usize),
    JumpIfFalse(usize),
    JumpIfTrue(usize),
    ReadInput,

    // Halt
    Halt,
}
