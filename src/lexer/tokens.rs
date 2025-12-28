#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    //MATH
    PLUS,
    MINUS,
    TIMES,
    DIVIDE,
    LEFTPAREN,
    RIGHTPAREN,
    OPENINGBRACE,
    CLOSINGBRACE,
    EQUAL,
    //VALUES
    NUMB,
    FLOAT,
    STRING,
    IDENTIFIER,
    //MISC
    COMMA,
    COLON,
    SEMICOLON,
    //VALUES
    TRUE,
    FALSE,
    //KEYWORDS
    FN,
    VAR,
    CONST,
    STR,
    IF,
    ELSE,
    //EOF
    EOF,
    VALUE,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub token_kind: TokenKind,
    pub token_value: String,
}
