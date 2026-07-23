use crate::backend::compiler::byte_code::Compilable;
use crate::backend::lexer::tokens::TokenKind;
use std::fmt;
use std::fmt::{Debug, Formatter};
use crate::backend::compiler::comptime_variable_checker::comptime_value_for_check::ComptimeValueType;
use crate::backend::ast::nodes::CallType;
/*
FunctionCallNode
*/

#[derive(Clone)]
pub struct FunctionCallNode {
    pub args: Vec<Box<dyn Compilable>>,
    pub name: String,
    pub call_type: CallType,
    pub return_type: Option<ComptimeValueType>,
}

impl Debug for FunctionCallNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.fmt_with_indent(f, 0)
    }
}

#[derive(Clone)]
pub struct ImportNode {
    pub module: String,
}

impl Debug for ImportNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.fmt_with_indent(f, 0)
    }
}
#[derive(Clone)]
pub struct ReturnNode{
    pub returns:Option<Box<dyn Compilable>>
}

impl Debug for ReturnNode {
   fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
       self.fmt_with_indent(f,0)
   } 
}

#[derive(Clone)]
pub struct LoopNode{
    pub body:Vec<Box<dyn Compilable>>
}
impl Debug for LoopNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.fmt_with_indent(f,0)
    }
}

/*
Binary Operation Node
*/
#[derive(Clone)]
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