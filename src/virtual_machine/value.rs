#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    StringValue(String),
    Number(f32),
    Bool(bool),
}
