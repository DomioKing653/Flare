use crate::compiler::comptime_variable_checker::comptime_value_for_check::{ComptimeValueType};
use std::collections::HashMap;
use crate::compiler::comptime_variable_checker::comptime_value_for_check::ComptimeValueType::{Number, StringValue};
use crate::errors::compiler_errors::CompileError;
use crate::errors::compiler_errors::CompileError::UndefinedType;
use crate::errors::parser_errors::ParserError;

pub struct CompileContext {
    pub variables: HashMap<String, ComptimeVariable>,
}
pub struct TypeError{
    unknown_type:String,
}
impl CompileContext{
    pub fn new()->Self{
        Self{
            variables:HashMap::new()
        }
    }
    pub fn get_type(type_to_identify:&str)->Result<ComptimeValueType,CompileError> {
        match type_to_identify {
            "numb"=>Ok(Number),
            "string"=>Ok(StringValue),
            _=>Err(
                UndefinedType {
                    undefined_type:type_to_identify.to_string()
                }
            )
        }
    }
}

pub struct ComptimeVariable {
    pub value_type: ComptimeValueType,
    pub is_const: bool,
}
