use std::collections::HashMap;
use crate::backend::compiler::comptime_variable_checker::comptime_value_for_check::ComptimeValueType;
use crate::backend::errors::compiler::compiler_errors::CompileError;
pub struct CompileTimeFunctionForCheck{
    pub return_type:ComptimeValueType,
    pub is_pub:bool

}
pub struct FunctionContext{
    pub functions:HashMap<String,CompileTimeFunctionForCheck>
}

impl Default for FunctionContext {
    fn default() -> Self {
        Self::new()
    }
}


impl FunctionContext {
    pub fn new()->Self{
        Self{
            functions:HashMap::new()
        }
    }
    pub fn add_function(&mut self,function: CompileTimeFunctionForCheck) -> Result<(),CompileError> {
        todo!();
    }
    
}
