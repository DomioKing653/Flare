use crate::{
    ast::{
        nodes::{
            VariableDefineNode,
            BinaryOpNode,
            FloatNode,
            NumberNode,
            ProgramNode,
            StringNode,
            VariableAccessNode
        }
    },
    lexer::{
        tokens::{
            TokenKind::{
                self,
                DIVIDE,
                EOF,
                FLOAT,
                IDENTIFIER,
                LEFTPAREN,
                MINUS,
                NUMB,
                PLUS,
                RIGHTPAREN,
                STRING,
                TIMES,
                VAR,
                CONST,
                VALUE,
                COLON,
                EQUAL
            },
            Token,
        }
    },
    errors::{
        parser_errors::{
            ParserError,
            ParserError::UnexpectedToken
        }
    },
    compiler::byte_code::Compilable,
};
use crate::ast::nodes::BoolNode;
use crate::lexer::tokens::TokenKind::{FALSE, TRUE};

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
    pub fn parse(&mut self)->Result<Box<dyn Compilable>,ParserError> {
        let mut program:ProgramNode=ProgramNode::new();
        while self.current_token().token_kind!=EOF {
            program.program_nodes.push(self.parse_stmt()?)
        }
        Ok(Box::new(program))
    }
    fn parse_stmt(&mut self)->Result<Box<dyn Compilable> ,ParserError>{
        match self.current_token().token_kind{
            VAR|CONST=>{
                self.parse_var_decl_stmt()
            }
            _=>self.parse_expr()
        }
    }
    fn parse_var_decl_stmt(&mut self)->Result<Box<dyn Compilable>,ParserError>{
        let is_const:bool;
        if self.current_token().token_kind==CONST {
            is_const = true;
        }else {
            is_const =false;
        }
        let id:String;
        self.advance();

        self.expect(IDENTIFIER)?;

        id=self.current_token().token_value.clone();

        self.advance();
        let mut value_type = None;

        if self.current_token().token_kind == COLON {
            self.advance();
            self.expect(IDENTIFIER)?;

            value_type = Some(self.current_token().token_value.clone());
            self.advance();
        }
        let value:Option<Box<dyn Compilable>>;
        if self.current_token().token_kind==EQUAL {
            self.advance();
            value=Some(self.parse_expr()?);
        }else{
            value=None
        }
        Ok(Box::new(VariableDefineNode{
            value_type,
            value,
            var_name:id,
            is_const
        }))
    }
    fn parse_expr(&mut self)->Result<Box<dyn Compilable>,ParserError>{
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
    fn parse_term(&mut self)->Result<Box<dyn Compilable>,ParserError>{
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
    fn parser_factor(&mut self)->Result<Box<dyn Compilable>,ParserError>{
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
        else if self.current_token().token_kind == TRUE ||self.current_token().token_kind == FALSE{
            let value = self.current_token().token_kind.clone();
            self.advance();
            Ok(Box::new(
                BoolNode{
                    value
                }
            ))

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
        else if self.current_token().token_kind==LEFTPAREN {
           self.advance();
           let value = self.parse_expr()?;
           self.expect(RIGHTPAREN)?;
            self.advance();
            Ok(value)
        }
        else if self.current_token().token_kind==STRING {
            let value = StringNode{
                value:self.current_token().token_value.clone()
            };
            self.advance();
            Ok(Box::new(value))
        }
        else {
            Err(UnexpectedToken {
                found:self.current_token().token_value.clone(),
                expected: VALUE
            })
        }
    }
    fn expect(&mut self,token_kind: TokenKind)->Result<Token,ParserError>{
        if self.current_token().token_kind==token_kind {
            Ok(self.current_token().clone())
        }else {
            Err(
                UnexpectedToken {
                    expected:token_kind,
                    found:self.current_token().token_value.clone()
                }
            )
        }
    }
}