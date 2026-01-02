#[derive(Debug, Clone, PartialEq)]
pub enum ComptimeValueType {
    Number,
    StringValue,
    Bool,
    Void,
    Array(Box<ComptimeValueType>),
}
