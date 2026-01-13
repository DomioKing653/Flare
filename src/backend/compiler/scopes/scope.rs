use crate::backend::compiler::comptime_variable_checker::comptime_context::ComptimeVariable;

pub struct Scope{
    scope_variables:Vec<ComptimeVariable>,
    next_scope:Option<Box<Scope>>,
    previos_scope:Option<Box<Scope>>
}

