use crate::compiler::comptime_variable_checker::comptime_value_for_check::ComptimeValueType::{
    self,
};
#[derive(Debug)]
pub struct FunctionArgs {
    pub argument_type: ComptimeValueType,
    pub name: String,
}
