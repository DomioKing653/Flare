use crate::backend::lexer::tokens::TokenKind;

#[derive(Debug)]
pub enum ParserError {
    UnexpectedToken { found: String, expected: TokenKind },
}
