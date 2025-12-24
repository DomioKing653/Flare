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
};
use crate::compiler::comptime_variable_checker::comptime_value_for_check::ComptimeValueType::StringValue;
use crate::compiler::instructions::Instructions::WriteLastOnStack;
use crate::errors::compiler_errors::CompileError::TypeMismatch;

pub trait Macro{
    fn compile(&self,out:&mut Compiler,args: &[Box<dyn Compilable>])->Result<ComptimeValueType,CompileError>;
    fn get_args(&self)->usize;
}

pub struct WriteLnMacro;


impl Macro for WriteLnMacro {
    fn compile(&self, compiler: &mut Compiler,args:&[Box<dyn Compilable>]) -> Result<ComptimeValueType, CompileError> {

        if args.len() !=self.get_args(){
            unreachable!()
        }else {

        }
        let type_of_args = args[0].compile(compiler)?;
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

    fn get_args(&self) -> usize {
        1
    }
}