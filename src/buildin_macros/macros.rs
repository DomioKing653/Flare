use std::{
    fmt,
    fmt::{Debug, Formatter}
};
use crate::{
    compiler::{
        comptime_variable_checker::{
            comptime_value_for_check::{
                ComptimeValueType::{
                    Null,
                    self
                }
            }
        },
        byte_code::{Compilable, Compiler},
    },
    errors::compiler_errors::CompileError,
    compiler::byte_code::indent_fn
};
use crate::compiler::comptime_variable_checker::comptime_value_for_check::ComptimeValueType::StringValue;
use crate::compiler::instructions::Instructions::WriteLastOnStack;
use crate::errors::compiler_errors::CompileError::TypeMismatch;

pub struct WriteLnMacro{
    pub args:Box<dyn Compilable>
}

impl Debug for WriteLnMacro {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.fmt_with_indent(f,0)
    }
}

impl Compilable for WriteLnMacro {
    fn compile(&self, compiler: &mut Compiler) -> Result<ComptimeValueType, CompileError> {
        let type_of_args = self.args.compile(compiler)?;
        if type_of_args!=StringValue {
            return Err(
                TypeMismatch {
                    expected:StringValue,
                    found:type_of_args
                }
            )
        };
        compiler.out.push(WriteLastOnStack);
        Ok(Null)
    }
    fn fmt_with_indent(&self, f: &mut Formatter<'_>, indent: usize) -> fmt::Result {
        writeln!(f, "{}WriteLn!({:?})",indent_fn(indent),self.args )
    }
}

