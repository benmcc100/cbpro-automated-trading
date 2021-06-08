use std::error::Error;
use std::fmt;

// Errors
#[derive(Debug)]
pub enum RequestError {
    InvalidRequest(String),
    InternalError(String),
    InvalidOrder(String),
    NetworkError,
}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid request!")
    }
}

impl Error for RequestError {}
