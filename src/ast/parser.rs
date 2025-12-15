use std::string::ParseError;
use crate::ast::nodes::{Node, ProgramNode};
use crate::lexer::tokens::Token;
pub struct Parser{
    tokens:Vec<Token>,
    token_idx:usize
}

impl Parser {
    pub fn new(token_list:Vec<Token>) ->Self{
        Self{
            tokens: token_list,
            token_idx:0
        }
    }
    pub fn parse(&self)->Result<Box<dyn Node>,ParseError> {
        Ok(Box::new(ProgramNode{
            program_nodes:Vec::new()
        }))
    }
}