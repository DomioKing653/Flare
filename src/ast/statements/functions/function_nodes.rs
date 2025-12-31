use std::fmt::Debug;

use crate::{
    ast::statements::functions::args_node::FunctionArgs,
    compiler::{
        byte_code::Compilable,
        comptime_variable_checker::comptime_value_for_check::ComptimeValueType,
    },
};
pub struct FunctionDefineNode {
    pub args: Vec<FunctionArgs>,
    pub id: String,
    pub body: Vec<Box<dyn Compilable>>,
    pub return_type: Option<ComptimeValueType>,
}

impl Compilable for FunctionDefineNode {
    fn compile(
        &self,
        compiler: &mut crate::compiler::byte_code::Compiler,
    ) -> Result<ComptimeValueType, crate::errors::compiler::compiler_errors::CompileError> {
        todo!()
    }

    fn fmt_with_indent(&self, f: &mut std::fmt::Formatter<'_>, indent: usize) -> std::fmt::Result {
        todo!()
    }
}

impl Debug for FunctionDefineNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FunctionDefineNode")
            .field("args", &self.args)
            .field("id", &self.id)
            .field("body", &self.body)
            .field("return_type", &self.return_type)
            .finish()
    }
}
