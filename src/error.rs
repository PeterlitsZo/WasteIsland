use std::fmt::{Debug, Display};

pub struct Error {
    message: String,
}

impl Error {
    pub fn new(message: &str) -> Self {
        Error {
            message: message.to_string(),
        }
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.message, f)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.message, f)
    }
}

pub trait ToInnerResult<T> {
    fn to_inner_result(self, prefix: &str) -> Result<T, Error>;
}

impl<T, E> ToInnerResult<T> for Result<T, E> where E: std::fmt::Display {
    fn to_inner_result(self, prefix: &str) -> Result<T, Error> {
        match self {
            Ok(v) => Ok(v),
            Err(err) => Err(Error::new(&format!("{}: {}", prefix, err.to_string())))
        }
    }
}
