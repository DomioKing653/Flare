#[derive(Debug, Clone, PartialEq)]
pub enum ComptimeValueType {
    Int,
    StringValue,
    Bool,
    Void,
    Float,
    Array(Box<ComptimeValueType>),
}
