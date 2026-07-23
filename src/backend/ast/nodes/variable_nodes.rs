use crate::backend::compiler::byte_code::Compilable;
use std::fmt;
use std::fmt::{Debug, Formatter};
/*
Variable Access
*/

#[derive(Clone)]
pub struct VariableAccessNode {
    pub variable_name: String,
}

impl fmt::Debug for VariableAccessNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.fmt_with_indent(f, 0)
    }
}
/*
Variable Define
*/
#[derive(Clone)]
pub struct VariableDefineNode {
    pub var_name: String,
    pub value_type: Option<String>,
    pub value: Option<Box<dyn Compilable>>,
    pub is_const: bool,
    pub is_public: bool,
}
impl Debug for VariableDefineNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.fmt_with_indent(f, 0)
    }
}

/*
Variable assign node
*/

#[derive(Clone)]
pub struct VariableAssignNode {
    pub name: String,
    pub value: Box<dyn Compilable>,
}

impl Debug for VariableAssignNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.fmt_with_indent(f, 0)
    }
}