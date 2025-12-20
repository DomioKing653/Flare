#[derive(Debug,Clone,PartialEq)]
pub enum TokenKind{
    //MATH
    PLUS,
    MINUS,
    TIMES,
    DIVIDE,
    LEFTPAREN,
    RIGHTPAREN,
    EQUAL,
    //VALUES
    NUMB,
    FLOAT,
    STRING,
    IDENTIFIER,
    COLON,
    //KEYWORDS
    FN,
    VAR,
    CONST,
    STR,
    //EOF
    EOF
}

#[derive(Clone)]
#[derive(Debug)]
pub struct Token{
    pub token_kind: TokenKind,
    pub token_value:String
}