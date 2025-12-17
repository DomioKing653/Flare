use crate::lexer::tokens::TokenKind;
use std::fmt;
use std::fmt::Formatter;
use crate::compiler::byte_code::Compilable;



pub struct ProgramNode {
    pub program_nodes: Vec<Box<dyn Compilable>>,
}

impl ProgramNode {
    pub fn new() -> Self {
        Self {
            program_nodes: Vec::new(),
        }
    }
}

impl fmt::Debug for ProgramNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.fmt_with_indent(f, 0)
    }
}
/*
Binary Operation Node
*/
pub struct BinaryOpNode {
    pub left: Box<dyn Compilable>,
    pub right: Box<dyn Compilable>,
    pub op_tok: TokenKind,
}

impl fmt::Debug for BinaryOpNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.fmt_with_indent(f, 0)
    }
}


/*
Number Node
*/
pub struct NumberNode {
    pub number: i32,
}

impl fmt::Debug for NumberNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.fmt_with_indent(f, 0)
    }
}

pub struct FloatNode {
    pub number: f32,
}

impl fmt::Debug for FloatNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.fmt_with_indent(f, 0)
    }
}

pub struct VariableAccessNode {
    pub variable_name: String,
}

impl fmt::Debug for VariableAccessNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.fmt_with_indent(f, 0)
    }
}
