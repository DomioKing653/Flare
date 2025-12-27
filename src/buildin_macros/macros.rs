use crate::compiler::instructions::Instructions::{ProcessExit, WriteLastOnStack};
use crate::{
    compiler::{
        byte_code::{Compilable, Compiler},
        comptime_variable_checker::comptime_value_for_check::ComptimeValueType::{
            self, Bool, Null, StringValue,
        },
        instructions::Instructions::WriteLnLastOnStack,
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
                StringValue | Number => compiler.out.push(WriteLnLastOnStack),
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
            let value = arg.compile(compiler)?;
            match value {
                StringValue | Number => compiler.out.push(WriteLastOnStack),
                Bool => {
                    return Err(TypeMismatch {
                        expected: StringValue,
                        found: Bool,
                    });
                }
                Null => {
                    return Err(TypeMismatch {
                        expected: StringValue,
                        found: Null,
                    });
                }
            }
        }
        Ok(Null)
    }
}

pub struct ProcessExitMacro;

impl Macro for ProcessExitMacro {
    fn compile(
        &self,
        out: &mut Compiler,
        args: &[Box<dyn Compilable>],
    ) -> Result<ComptimeValueType, CompileError> {
        if args.len() != 1 {
            return Err(CompileError::WrongMacroArgCount {
                expected: 1,
                found: args.len(),
            });
        } else {
            let value = args[0].compile(out)?;
            match value {
                Number => {
                    out.out.push(ProcessExit);
                    return Ok(Null);
                }
                _ => Err(TypeMismatch {
                    expected: Number,
                    found: value,
                }),
            }
        }
    }
}
