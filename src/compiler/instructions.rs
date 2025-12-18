#[derive(Debug)]
pub enum Instructions{
    PushNumber(f32),
    PushString(String),
    Add,
    Sub,
    Div,
    Mul,
    Halt
    
}