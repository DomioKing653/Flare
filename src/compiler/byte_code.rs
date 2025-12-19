use std::fmt;
use std::fmt::Formatter;
use std::io::Error;
use crate::ast::nodes::{BinaryOpNode, FloatNode, NumberNode, ProgramNode, StringNode, VariableAccessNode, VariableDefineNode};
use crate::compiler::comptime_variable_checker::comptime_context::CompileContext;
use crate::compiler::comptime_variable_checker::comptime_value_for_check::ComptimeValueType;
use crate::compiler::comptime_variable_checker::comptime_value_for_check::ComptimeValueType::{Null, Number, StringValue};
use crate::compiler::instructions::Instructions;
use crate::compiler::instructions::Instructions::{Add, Div, Halt, LoadVar, Mul, PushString, Sub};
use crate::errors::compiler_errors::CompileError;
use crate::lexer::tokens::TokenKind;

pub trait Compilable : fmt::Debug{
    fn compile(&self,compiler:&mut Compiler)->Result<ComptimeValueType,CompileError>;
    fn fmt_with_indent(&self, f: &mut Formatter<'_>, indent: usize) -> fmt::Result;
}
fn indent_fn(n: usize) -> String {
    "  ".repeat(n)
}

pub struct Compiler{
    pub context:CompileContext,
    pub out:Vec<Instructions>
}

impl Compiler {
    pub fn new()->Self{
        Self{
            context:CompileContext::new(),
            out:Vec::new()
        }
    }
}

impl Compilable for NumberNode{
    fn compile(&self, compiler: &mut Compiler) -> Result<ComptimeValueType, CompileError> {
        compiler.out.push(Instructions::PushNumber(self.number as f32));
        Ok(Number)
    }
    fn fmt_with_indent(&self, f: &mut Formatter<'_>, indent: usize) -> fmt::Result {
        writeln!(f, "{}Number({})", indent_fn(indent), self.number)
    }
}

impl Compilable for FloatNode{
    fn compile(&self, out: &mut Compiler) -> Result<ComptimeValueType, CompileError> {
        out.out.push(Instructions::PushNumber(self.number));
        Ok(Number)
    }
    fn fmt_with_indent(&self, f: &mut Formatter<'_>, indent: usize) -> fmt::Result {
        writeln!(f, "{}Float({})", indent_fn(indent), self.number)
    }
}

impl Compilable for BinaryOpNode {
    fn compile(&self, compiler: &mut Compiler) -> Result<ComptimeValueType, CompileError> {
        let right = self.left.compile(compiler)?;
        let left = self.right.compile(compiler)?;
        match self.op_tok {
            TokenKind::PLUS => match (&left, &right) {
                (Number, Number) => {
                    compiler.out.push(Add);
                    Ok(Number)
                }
                (StringValue, StringValue) => {
                    compiler.out.push(Add);
                    Ok(StringValue)
                }
                _ => Err(CompileError::InvalidBinaryOp {
                    op: "+",
                    left,
                    right,
                }),
            },

            TokenKind::MINUS => {
                if let Number = right {
                    compiler.out.push(Sub);
                    Ok(Number)
                } else {
                    Err(CompileError::InvalidBinaryOp {
                        op: "/",
                        left,
                        right,
                    })
                }
            }
            TokenKind::TIMES => {
                if let Number = right {
                    compiler.out.push(Mul);
                    Ok(Number)
                } else {
                    Err(CompileError::InvalidBinaryOp {
                        op: "/",
                        left,
                        right,
                    })
                }
            }
            TokenKind::DIVIDE => {
                if let Number = right {
                    compiler.out.push(Div);
                    Ok(Number)
                } else {
                    Err(CompileError::InvalidBinaryOp {
                        op: "/",
                        left,
                        right,
                    })
                }
            }
            _ => {
                unreachable!()
            }
        }
    }
    fn fmt_with_indent(&self, f: &mut Formatter<'_>, indent: usize) -> fmt::Result {
        writeln!(f, "{}BinaryOp({:?})", indent_fn(indent), self.op_tok)?;
        self.left.fmt_with_indent(f, indent + 1)?;
        self.right.fmt_with_indent(f, indent + 1)?;
        Ok(())
    }
}
impl Compilable for ProgramNode {
    fn compile(&self, compiler: &mut Compiler) -> Result<ComptimeValueType, CompileError> {
        for program_node in &self.program_nodes {
            program_node.compile(compiler)?;
        }
        compiler.out.push(Halt);
        Ok(Null)
    }
    fn fmt_with_indent(&self, f: &mut Formatter<'_>, indent: usize) -> fmt::Result {
        writeln!(f, "{}Program", indent_fn(indent))?;
        for node in &self.program_nodes {
            node.fmt_with_indent(f, indent + 1)?;
        }
        Ok(())
    }
}

impl Compilable for VariableAccessNode {
    fn compile(&self, compiler: &mut Compiler) -> Result<ComptimeValueType, CompileError> {
        compiler.out.push(LoadVar(self.variable_name.to_string()));
        Ok(Null)
    }
    fn fmt_with_indent(&self, f: &mut Formatter<'_>, indent: usize) -> fmt::Result {
        writeln!(f, "{}Var({})", indent_fn(indent), self.variable_name)
    }
}

impl Compilable for StringNode {
    fn compile(&self, compiler: &mut Compiler) -> Result<ComptimeValueType, CompileError> {
        compiler.out.push(PushString(self.value.clone()));
        Ok(StringValue)
    }
    fn fmt_with_indent(&self, f: &mut Formatter<'_>, indent: usize) -> fmt::Result {
        writeln!(f, "{}String({})", indent_fn(indent), self.value)
    }
}

impl Compilable for VariableDefineNode {
    fn compile(&self, compiler: &mut Compiler) -> Result<ComptimeValueType, CompileError> {
        if let Some(value) = &self.value{
            value.compile(compiler)?;
        }
        compiler.out.push(Instructions::SaveVar(self.var_name.clone()));
        Ok(Null)
    }
    fn fmt_with_indent(&self, f: &mut Formatter<'_>, indent: usize) -> fmt::Result {
        write!(f, "{}var:{:?}=", indent,self.value_type)?;
        if let Some(value) = &self.value {
            value.fmt_with_indent(f, 0)?;
        } else {
            write!(f, "None")?;
        }
        Ok(())
    }
}