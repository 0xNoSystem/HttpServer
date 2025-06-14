use thiserror::Error;


#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to parse request")]
    ParseError,

    #[error("Invalid HTTP method")]
    InvalidMethod,

    #[error("Invalid request line format")]
    InvalidRequestLine,

    #[error("Unsupported HTTP version: {0}")]
    UnsupportedHttpVersion(String),

    #[error("Incomplete request (connection closed early)")]
    IncompleteRequest,

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("UTF-8 error: {0}")]
    Utf8(#[from] std::str::Utf8Error),
    // Additional variants omitted for brevity
}

