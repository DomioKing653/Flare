use crate::backend::compiler::comptime_variable_checker::comptime_value_for_check::ComptimeValueType;
use crate::backend::errors::compiler::compiler_errors::CompileError;
use crate::backend::compiler::comptime_variable_checker::comptime_value_for_check::ComptimeValueType::{
    Bool, Int, StringValue, Void, Float
};
use crate::backend::compiler::comptime_variable_checker::functions::Function;
use crate::backend::errors::compiler::compiler_errors::CompileError::UndefinedType;
use std::collections::HashMap;

pub struct CompileContext {
    pub variables: HashMap<String, ComptimeVariable>,
    pub functions: HashMap<String, Function>,
}
impl CompileContext {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }
    pub fn get_type(type_to_identify: &str) -> Result<ComptimeValueType, CompileError> {
        match type_to_identify {
            "numb" => Ok(Int),
            "string" => Ok(StringValue),
            "bool" => Ok(Bool),
            "void" => Ok(Void),
            "flt" => Ok(Float),
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
