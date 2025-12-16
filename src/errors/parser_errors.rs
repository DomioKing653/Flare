use crate::lexer::tokens::Token;

pub enum ParserErrorType{
    UnexpectedTokenAtFactor
}

pub struct ParserError{
    pub error_type:ParserErrorType,
    pub wrong_token:Token
}