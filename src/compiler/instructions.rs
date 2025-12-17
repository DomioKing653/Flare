#[derive(Debug)]
pub enum Instructions{
    PushNumber(f32),
    Add,
    Sub,
    Div,
    Mul,
    Halt
    
}