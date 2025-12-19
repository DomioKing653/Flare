use crate::compiler::comptime_variable_checker::comptime_value_for_check::{ComptimeValueType};
use std::collections::HashMap;

pub struct CompileContext {
    pub variables: HashMap<String, ComptimeVariable>,
}
impl CompileContext{
    pub fn new()->Self{
        Self{
            variables:HashMap::new()
        }
    }
}

pub struct ComptimeVariable {
    pub value_type: ComptimeValueType,
    pub is_const: bool,
}
