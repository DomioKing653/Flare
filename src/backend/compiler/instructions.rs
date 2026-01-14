
#[repr(u32)]
#[derive(Debug, Clone,PartialEq,)]
pub enum Instructions {
    Add = 0,
    Sub = 1,
    Div = 2,
    Mul = 3,
    Modulo = 4,
    //Comparison
    GreaterThan = 5,
    LessThan = 6,
    Equal = 7,
    //Variables
    LoadVar(String) = 15,
    SaveVar(String) = 16,
    //Values
    PushString(String) = 20,
    PushBool(bool) = 21,
    PushNumber(f32) = 22,
    ReadInput = 32,
    //Printing
    WriteLnLastOnStack = 30,
    WriteLastOnStack = 31,
    //Process
    ProcessExit = 35,
    //Control flow
    Jump(usize) = 40,
    JumpIfFalse(usize) = 41,
    JumpIfTrue(usize) = 42,


    // Halt
    Halt = 255,
}
