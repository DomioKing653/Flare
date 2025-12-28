use crate::compiler::comptime_variable_checker::comptime_value_for_check::ComptimeValueType;
use crate::compiler::comptime_variable_checker::comptime_value_for_check::ComptimeValueType::{
    Bool, Number, StringValue,
};
use crate::errors::compiler::compiler_errors::CompileError;
use crate::errors::compiler::compiler_errors::CompileError::UndefinedType;
use std::collections::HashMap;

pub struct CompileContext {
    pub variables: HashMap<String, ComptimeVariable>,
}
impl CompileContext {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }
    pub fn get_type(type_to_identify: &str) -> Result<ComptimeValueType, CompileError> {
        match type_to_identify {
            "numb" => Ok(Number),
            "string" => Ok(StringValue),
            "bool" => Ok(Bool),
            _ => Err(UndefinedType {
                undefined_type: type_to_identify.to_string(),
            }),
        }
    }
}

pub struct ComptimeVariable {
    pub value_type: ComptimeValueType,
    pub is_const: bool,
}
