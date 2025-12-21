#[derive(Debug,Clone)]
pub enum Value{
    StringValue(String),
    Number(f32),
    Bool(bool)
}

