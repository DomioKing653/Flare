use crate::compiler::instructions::Instructions::{ProcessExit, ReadInput, WriteLastOnStack};
use crate::{
    compiler::{
        byte_code::{Compilable, Compiler},
        comptime_variable_checker::comptime_value_for_check::ComptimeValueType::{
            self, Bool, StringValue, Void,
        },
        instructions::Instructions::WriteLnLastOnStack,
    },
    errors::compiler::compiler_errors::CompileError::{self, TypeMismatch},
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
                    return Err(CompileError::ExpectedPrintable { found: Bool });
                }
                Void => {
                    return Err(CompileError::ExpectedPrintable { found: Void });
                }
            }
        }
        Ok(Void)
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
                    return Err(CompileError::ExpectedPrintable { found: Bool });
                }
                Void => {
                    return Err(CompileError::ExpectedPrintable { found: Void });
                }
            }
        }
        Ok(Void)
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
                    return Ok(Void);
                }
                _ => Err(TypeMismatch {
                    expected: Number,
                    found: value,
                }),
            }
        }
    }
}

pub struct ReadInputMacro;

impl Macro for ReadInputMacro {
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
                StringValue => {
                    out.out.push(WriteLastOnStack);
                    out.out.push(ReadInput);
                    return Ok(StringValue);
                }
                _ => Err(TypeMismatch {
                    expected: StringValue,
                    found: value,
                }),
            }
        }
    }
}
