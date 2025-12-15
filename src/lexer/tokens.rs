#[derive(Clone)]
#[derive(Debug)]
pub enum TokenKind{
    //MATH
    PLUS,
    MINUS,
    TIMES,
    DIVIDE,
    //VALUES
    NUMB,
    FLOAT,
    STRING,
    IDENTIFIER,
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