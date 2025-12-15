#[derive(Debug)]
pub enum LexerErrorType{
    UnknownTokenError,
    MoreDotInANumberError
}
#[derive(Debug)]
pub struct LexerError{
    pub wrong_token:String,
    pub error_type: LexerErrorType
}