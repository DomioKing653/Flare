use ComptimeValueType::Number;
use crate::{
    compiler::{
        comptime_variable_checker::{
            comptime_value_for_check::{
                ComptimeValueType::{
                    Null,
                    self,
                    Bool,
                    StringValue
                }
            }
        },
        byte_code::{Compilable, Compiler},
        instructions::Instructions::WriteLastOnStack
    },
    errors::{
        compiler_errors::{
            CompileError::{
                self,
                TypeMismatch
            }
        }
    }
};

pub trait Macro{
    fn compile(&self,out:&mut Compiler,args: &[Box<dyn Compilable>])->Result<ComptimeValueType,CompileError>;
}

pub struct WriteLnMacro;


impl Macro for WriteLnMacro {
    fn compile(&self, compiler: &mut Compiler,args:&[Box<dyn Compilable>]) -> Result<ComptimeValueType, CompileError> {
        for arg in args {
            let value = arg.compile(compiler)?;
            match value {
                StringValue | Number => {
                    compiler.out.push(WriteLastOnStack)
                }
                Bool =>{
                    return  Err(
                        TypeMismatch {
                            expected:StringValue,
                            found:Bool
                        }
                    )
                }
                Null=>{
                    unreachable!()
                }
            }
        }
        Ok(Null)
    }
}