use derive_more::{Display, From};

pub type Result<T> = ::core::result::Result<T, Error>;

#[derive(Debug, Display, From)]
pub enum Error {
    #[display(fmt = "Unexpected token: expected {}, found {}", expected, found)]
    UnexpectedToken { expected: String, found: String },

    #[from]
    Custom(String),
    #[from]
    Io(std::io::Error),
}

impl Error {
    pub fn custom(error: impl Into<String>) -> Self {
        Error::Custom(error.into())
    }
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Error::Custom(s.to_string())
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_error() {
        use super::*;
        let error = Error::custom("Custom error");
        assert_eq!(error.to_string(), "Custom error");
    }
}
