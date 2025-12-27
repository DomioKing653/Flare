#[derive(Debug, Clone)]
pub enum Instructions {
    Add,
    Sub,
    Div,
    Mul,
    PushString(String),
    LoadVar(String),
    SaveVar(String),
    PushBool(bool),
    PushNumber(f32),
    WriteLnLastOnStack,
    WriteLastOnStack,
    If(usize),
    Halt,
}
