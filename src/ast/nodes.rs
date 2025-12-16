use crate::ast::value_node::ValueNode;
use crate::ast::value_node::ValueType::{NUMBER};
use crate::lexer::tokens::TokenKind;
use std::fmt;
pub trait Node:fmt::Debug {
    fn visit_node(&mut self) -> ValueNode;
    fn fmt_with_indent(&self, f: &mut fmt::Formatter<'_>, indent: usize) -> fmt::Result;
}

fn indent_fn(n: usize) -> String {
    "  ".repeat(n)
}

pub struct ProgramNode {
    pub program_nodes: Vec<Box<dyn Node>>,
}

impl ProgramNode {
    pub fn new() -> Self {
        Self {
            program_nodes: Vec::new(),
        }
    }
}

impl Node for ProgramNode {
    fn visit_node(&mut self) -> ValueNode {
        self.program_nodes[0].visit_node()
    }

    fn fmt_with_indent(&self, f: &mut fmt::Formatter<'_>, indent: usize) -> fmt::Result {
        writeln!(f, "{}Program", indent_fn(indent))?;
        for node in &self.program_nodes {
            node.fmt_with_indent(f, indent + 1)?;
        }
        Ok(())
    }
}

impl fmt::Debug for ProgramNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_with_indent(f, 0)
    }
}

pub struct BinaryOpNode {
    pub left: Box<dyn Node>,
    pub right: Box<dyn Node>,
    pub op_tok: TokenKind,
}

impl Node for BinaryOpNode {
    fn visit_node(&mut self) -> ValueNode {
        match self.op_tok {
            TokenKind::PLUS => ValueNode{
                value_type:NUMBER,
                number:Some(self.left.visit_node().number.unwrap() + self.right.visit_node().number.unwrap()),
                bool:None,
                string:None},
            TokenKind::MINUS => ValueNode{
                value_type:NUMBER,
                number:Some(self.left.visit_node().number.unwrap() - self.right.visit_node().number.unwrap()),
                bool:None,
                string:None},
            TokenKind::TIMES => ValueNode{
                value_type:NUMBER,
                number:Some(self.left.visit_node().number.unwrap() * self.right.visit_node().number.unwrap()),
                bool:None,
                string:None},
            TokenKind::DIVIDE => ValueNode{
                value_type:NUMBER,
                number:Some(self.left.visit_node().number.unwrap() + self.right.visit_node().number.unwrap()),
                bool:None,
                string:None},
            _=>unreachable!()
        }
    }

    fn fmt_with_indent(&self, f: &mut fmt::Formatter<'_>, indent: usize) -> fmt::Result {
        writeln!(f, "{}BinaryOp({:?})", indent_fn(indent), self.op_tok)?;
        self.left.fmt_with_indent(f, indent + 1)?;
        self.right.fmt_with_indent(f, indent + 1)?;
        Ok(())
    }
}

impl fmt::Debug for BinaryOpNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_with_indent(f, 0)
    }
}

pub struct NumberNode {
    pub number: i32,
}

impl Node for NumberNode {
    fn visit_node(&mut self) -> ValueNode {
        ValueNode {
            value_type: NUMBER,
            string: None,
            bool: None,
            number: Some(self.number as f32),
        }
    }

    fn fmt_with_indent(&self, f: &mut fmt::Formatter<'_>, indent: usize) -> fmt::Result {
        writeln!(f, "{}Number({})", indent_fn(indent), self.number)
    }
}

impl fmt::Debug for NumberNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_with_indent(f, 0)
    }
}

pub struct FloatNode {
    pub number: f32,
}

impl Node for FloatNode {
    fn visit_node(&mut self) -> ValueNode {
        ValueNode {
            value_type: NUMBER,
            string: None,
            bool: None,
            number: Some(self.number),
        }
    }

    fn fmt_with_indent(&self, f: &mut fmt::Formatter<'_>, indent: usize) -> fmt::Result {
        writeln!(f, "{}Float({})", indent_fn(indent), self.number)
    }
}

impl fmt::Debug for FloatNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_with_indent(f, 0)
    }
}

pub struct VariableAccessNode {
    pub variable_name: String,
}

impl Node for VariableAccessNode {
    fn visit_node(&mut self) -> ValueNode {
        todo!()
    }

    fn fmt_with_indent(&self, f: &mut fmt::Formatter<'_>, indent: usize) -> fmt::Result {
        writeln!(f, "{}Var({})", indent_fn(indent), self.variable_name)
    }
}

impl fmt::Debug for VariableAccessNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_with_indent(f, 0)
    }
}
