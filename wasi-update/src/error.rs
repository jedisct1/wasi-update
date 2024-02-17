#[derive(Debug, thiserror::Error)]
pub enum WSError {
    #[error("Unsupported module type")]
    UnsupportedModuleType,

    #[error("Parse error")]
    ParseError,

    #[error("I/O error")]
    IOError(#[from] std::io::Error),

    #[error("EOF")]
    Eof,

    #[error("UTF-8 error")]
    UTF8Error(#[from] std::str::Utf8Error),
}
