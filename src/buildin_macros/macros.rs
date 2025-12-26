use crate::{
    compiler::{
        byte_code::{Compilable, Compiler},
        comptime_variable_checker::comptime_value_for_check::ComptimeValueType::{
            self, Bool, Null, StringValue,
        },
        instructions::Instructions::{self, WriteLnLastOnsStack},
    },
    errors::compiler_errors::CompileError::{self, TypeMismatch},
};
use ComptimeValueType::Number;

pub trait Macro {
    fn compile(
        &self,
        out: &mut Compiler,
        args: &[Box<dyn Compilable>],
    ) -> Result<ComptimeValueType, CompileError>;
}

pub struct WriteLnMacro;

impl Macro for WriteLnMacro {
    fn compile(
        &self,
        compiler: &mut Compiler,
        args: &[Box<dyn Compilable>],
    ) -> Result<ComptimeValueType, CompileError> {
        for arg in args {
            let value = arg.compile(compiler)?;
            match value {
                StringValue | Number => compiler.out.push(WriteLnLastOnsStack),
                Bool => {
                    return Err(TypeMismatch {
                        expected: StringValue,
                        found: Bool,
                    });
                }
                Null => {
                    unreachable!()
                }
            }
        }
        Ok(Null)
    }
}

pub struct WriteMacro;

impl Macro for WriteMacro {
    fn compile(
        &self,
        compiler: &mut Compiler,
        args: &[Box<dyn Compilable>],
    ) -> Result<ComptimeValueType, CompileError> {
        for arg in args {
            arg.compile(compiler)?;
            compiler.out.push(Instructions::WriteLastOnStack);
        }
        Ok(Null)
    }
}
