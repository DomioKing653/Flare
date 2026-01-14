use crate::backend::ast::statements::functions::args_node::FunctionArgs;
use crate::backend::compiler::byte_code::Compilable;
use crate::backend::compiler::comptime_variable_checker::comptime_value_for_check::ComptimeValueType;
pub struct CompileTimeFunctionForCheck{
    pub return_type:ComptimeValueType,
    pub is_pub:bool,
    pub body:Vec<Box<dyn Compilable>>,
    pub args:Vec<FunctionArgs>

}

