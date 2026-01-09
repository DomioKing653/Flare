use crate::backend::compiler::byte_code::Compilable;
use crate::backend::compiler::comptime_variable_checker::comptime_value_for_check::ComptimeValueType;
pub struct Function {
    pub ret_type: ComptimeValueType,
    pub args: Vec<Box<dyn Compilable>>,
}
