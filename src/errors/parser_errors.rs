use crate::lexer::tokens::Token;

pub enum ParserErrorType{
    UnexpectedTokenAtFactor,
    ExpectedClosingParen,
    ExpectedId,
    ExpectedExplicitType
}

pub struct ParserError{
    pub error_type:ParserErrorType,
    pub wrong_token:Token
}