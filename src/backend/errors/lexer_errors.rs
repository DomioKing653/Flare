use thiserror::Error;
#[derive(Debug, Error)]
pub enum LexerError {
    #[error("Unknown token: {wrong_token}")]
    UnknownToken { wrong_token: String },
    #[error("Cannot have more dots in a number")]
    MoreDotInANumber,
    #[error("Cannot parse empty file")]
    EmptyFile,
}
