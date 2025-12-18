use crate::lexer::tokens::TokenKind;
use std::fmt;
use std::fmt::{Debug, Formatter, Pointer};
use crate::compiler::byte_code::Compilable;
use crate::compiler::comptime_variable_checker::comptime_value_for_check::ComptimeValueType;

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
/*
Float node
*/
pub struct FloatNode {
    pub number: f32,
}

impl fmt::Debug for FloatNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.fmt_with_indent(f, 0)
    }
}
/*
String node
*/

pub struct StringNode{
    pub value:String
}
impl Debug for StringNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.fmt_with_indent(f, 0)
    }
}
//VARIABLES
/*
Variable Access
*/

pub struct VariableAccessNode {
    pub variable_name: String,
}

impl fmt::Debug for VariableAccessNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.fmt_with_indent(f, 0)
    }
}
/*
Variable Define
*/
pub struct VariableDefineNode{
    pub var_name:String,
    pub value_type:ComptimeValueType,
    pub value:Option<Box<dyn Compilable>>
}
impl Debug for VariableDefineNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.fmt_with_indent(f,0)
    }
}