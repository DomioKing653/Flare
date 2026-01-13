use std::fmt::Debug;

use crate::backend::{
    ast::statements::functions::args_node::FunctionArgs,
    compiler::{
        byte_code::{Compilable, Compiler}, comptime_variable_checker::comptime_value_for_check::ComptimeValueType, functions_compiler_context::CompileTimeFunctionForCheck
    },
    errors::compiler::compiler_errors::CompileError,
};
pub struct FunctionDefineNode {
    pub args: Vec<FunctionArgs>,
    pub id: String,
    pub body: Vec<Box<dyn Compilable>>,
    pub return_type: Option<ComptimeValueType>,
}

impl Compilable for FunctionDefineNode {
    fn compile(&self, compiler: &mut Compiler) -> Result<ComptimeValueType, CompileError> {
        let return_type = self.return_type.clone().unwrap();
        compiler.function_context.add_function(
            CompileTimeFunctionForCheck{
                is_pub:true,
                return_type
            }
        )?;
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
