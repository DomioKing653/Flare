use crate::lexer::tokens::Token;

pub enum ParserErrorType{

}

pub struct ParserError{
    error_type:ParserErrorType,
    wrong_token:Token
}