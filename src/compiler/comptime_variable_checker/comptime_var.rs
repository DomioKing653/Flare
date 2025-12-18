use crate::compiler::comptime_variable_checker::comptime_value_for_check::{ComptimeValueType};

pub struct ComptimeVariable{
    value_type:ComptimeValueType,
    name:String
}