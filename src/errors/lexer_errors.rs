#[derive(Debug)]
pub enum LexerErrorType{
    UnknownTokenError,
    MoreDotInANumberError,
    EmptyFile
}
#[derive(Debug)]
pub struct LexerError{
    pub wrong_token:String,
    pub error_type: LexerErrorType
}