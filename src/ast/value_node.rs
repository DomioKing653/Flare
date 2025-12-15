pub enum ValueType{
    STRING,
    FLOAT,
    NUMBER,
    BOOL
}
pub struct ValueNode{
    pub value_type: ValueType,
    pub string: Option<String>,
    pub bool:Option<bool>,
    pub int:Option<i32>,
    pub float:Option<f32>
}