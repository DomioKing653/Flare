use crate::ast::nodes::{BinaryOpNode, FloatNode, Node, NumberNode, ProgramNode, VariableAccessNode};
use crate::errors::parser_errors::ParserError;
use crate::errors::parser_errors::ParserErrorType::UnexpectedTokenAtFactor;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::TokenKind::{DIVIDE, EOF, FLOAT, IDENTIFIER, MINUS, NUMB, PLUS, TIMES};

pub struct Parser{
    tokens:Vec<Token>,
    token_idx:usize,
}

impl Parser {
    pub fn new(token_list:Vec<Token>) ->Self{
        Self{
            tokens: token_list,
            token_idx:0,
        }
    }
    pub fn current_token(&self)->&Token{
        &self.tokens[self.token_idx]
    }
    pub fn advance(&mut self){
        self.token_idx+=1;
    }
    pub fn parse(&mut self)->Result<Box<dyn Node>,ParserError> {
        let mut program:ProgramNode=ProgramNode::new();
        while self.current_token().token_kind!=EOF {
            program.program_nodes.push(self.parse_stmt()?)
        }
        Ok(Box::new(program))
    }
    fn parse_stmt(&mut self)->Result<Box<dyn Node> ,ParserError>{
        match self.current_token(){
            _=>self.parse_expr()
        }
    }
    fn parse_expr(&mut self)->Result<Box<dyn Node>,ParserError>{
        let mut term =self.parse_term()?;
        while self.current_token().token_kind==PLUS||self.current_token().token_kind==MINUS {
            let operator=self.current_token().token_kind.clone();
            self.advance();
            term = Box::new(BinaryOpNode{
                left: term,
                right:self.parse_term()?,
                op_tok:operator
            });
        }
        Ok(term)
    }
    fn parse_term(&mut self)->Result<Box<dyn Node>,ParserError>{
        let mut factor =self.parser_factor()?;
        while self.current_token().token_kind==TIMES||self.current_token().token_kind==DIVIDE {
            let operator=self.current_token().token_kind.clone();
            self.advance();
            factor = Box::new(BinaryOpNode{
                left: factor,
                right:self.parser_factor()?,
                op_tok:operator
            });
        }
        Ok(factor)
    }
    fn parser_factor(&mut self)->Result<Box<dyn Node>,ParserError>{
        if self.current_token().token_kind == FLOAT {
            let value = match self.current_token().token_value.parse::<f32>() {
                Err(_) => unreachable!(),
                Ok(numb) => numb
            };
            self.advance();
            Ok(Box::new(FloatNode{
                number:value
            }))
        }
        else if self.current_token().token_kind == NUMB {
            let value = match self.current_token().token_value.parse::<i32>() {
                Ok(numb)=>numb,
                Err(_)=>unreachable!()
            };
            self.advance();
            Ok(Box::new(
                NumberNode{
                    number:value
                }
            ))
        }
        else if self.current_token().token_kind==IDENTIFIER {
            let value = self.current_token().token_value.clone();
            self.advance();
            Ok(Box::new(
                VariableAccessNode{
                    variable_name:value
                }
            ))
        }
        else {
            Err(ParserError{
                wrong_token:self.current_token().clone(),
                error_type:UnexpectedTokenAtFactor
            })
        }
    }
}