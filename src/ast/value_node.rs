#[derive(Debug)]
pub enum ValueType{
    STRING,
    NUMBER,
    BOOL,
    NULL
}
#[derive(Debug)]
pub struct ValueNode{
    pub value_type: ValueType,
    pub string: Option<String>,
    pub bool:Option<bool>,
    pub number:Option<f32>
}