use std::fmt;
use std::fmt::Formatter;
use crate::ast::nodes::{BinaryOpNode, FloatNode, NumberNode, ProgramNode, VariableAccessNode};
use crate::compiler::instructions::Instructions;
use crate::compiler::instructions::Instructions::{Add, Div, Halt, Mul, Sub};
use crate::lexer::tokens::TokenKind;
pub trait Compilable : fmt::Debug{
    fn compile(&self,out: &mut Vec<Instructions>);
    fn fmt_with_indent(&self, f: &mut Formatter<'_>, indent: usize) -> fmt::Result;
}
fn indent_fn(n: usize) -> String {
    "  ".repeat(n)
}
impl Compilable for NumberNode{
    fn compile(&self, out: &mut Vec<Instructions>) {
        out.push(Instructions::PushNumber(self.number as f32))
    }
    fn fmt_with_indent(&self, f: &mut Formatter<'_>, indent: usize) -> fmt::Result {
        writeln!(f, "{}Number({})", indent_fn(indent), self.number)
    }
}

impl Compilable for FloatNode{
    fn compile(&self, out: &mut Vec<Instructions>) {
        out.push(Instructions::PushNumber(self.number))
    }
    fn fmt_with_indent(&self, f: &mut Formatter<'_>, indent: usize) -> fmt::Result {
        writeln!(f, "{}Float({})", indent_fn(indent), self.number)
    }
}

impl Compilable for BinaryOpNode {
    fn compile(&self, out: &mut Vec<Instructions>) {
        self.left.compile(out);
        self.right.compile(out);
        match self.op_tok {
            TokenKind::PLUS =>out.push(Add),
            TokenKind::MINUS=>out.push(Sub),
            TokenKind::DIVIDE=>out.push(Div),
            TokenKind::TIMES=>out.push(Mul),
            _=> unreachable!()
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
    fn compile(&self, out: &mut Vec<Instructions>) {
        for program_node in &self.program_nodes {
            program_node.compile(out)
        }
        out.push(Halt);
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
    fn compile(&self, out: &mut Vec<Instructions>) {
        todo!()
    }
    fn fmt_with_indent(&self, f: &mut Formatter<'_>, indent: usize) -> fmt::Result {
        writeln!(f, "{}Var({})", indent_fn(indent), self.variable_name)
    }
}