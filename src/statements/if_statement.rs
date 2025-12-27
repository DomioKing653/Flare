use crate::compiler::byte_code::indent_fn;
use crate::compiler::instructions::Instructions;
use crate::errors::compiler_errors::CompileError::TypeMismatch;
use crate::statements::if_statement::ComptimeValueType::{Bool, Null};
use crate::{
    compiler::{
        byte_code::Compilable,
        comptime_variable_checker::comptime_value_for_check::ComptimeValueType,
    },
    errors::compiler_errors::CompileError,
};
use std::fmt::Debug;

pub struct IfStatement {
    pub statements: Vec<Box<dyn Compilable>>,
    pub expr: Box<dyn Compilable>,
    pub else_stmt: Option<ElseStatement>,
}

impl Compilable for IfStatement {
    fn compile(
        &self,
        compiler: &mut crate::compiler::byte_code::Compiler,
    ) -> Result<ComptimeValueType, CompileError> {
        match self.expr.compile(compiler)? {
            type_of_expr if type_of_expr != Bool => {
                return Err(TypeMismatch {
                    expected: Bool,
                    found: type_of_expr,
                });
            }
            _ => (),
        }
        compiler.out.push(Instructions::If(self.statements.len()));
        Ok(Null)
    }
    fn fmt_with_indent(&self, f: &mut std::fmt::Formatter<'_>, indent: usize) -> std::fmt::Result {
        writeln!(f, "{}if(...)", indent_fn(indent))?;
        let mut i = 0;
        while i < self.statements.len() {
            self.statements[i].fmt_with_indent(f, indent)?;
            i += 1;
        }
        Ok(())
    }
}

impl Debug for IfStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_with_indent(f, 0)
    }
}

pub struct ElseStatement {
    pub statemnt: Vec<Box<dyn Compilable>>,
}

impl Debug for ElseStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Else")
            .field("statemnt", &self.statemnt)
            .finish()
    }
}
