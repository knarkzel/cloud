use serde::{Serialize, Deserialize};

// Error type
#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    description: String,
}

impl Error {
    pub fn new<T: std::fmt::Display>(message: T) -> Self {
        Self {
            description: message.to_string(),
        }
    }
}

impl<T: std::fmt::Display> From<T> for Error {
    fn from(value: T) -> Self {
        Self {
            description: value.to_string(),
        }
    }
}

// Result type
pub type Result<T, E = Error> = std::result::Result<T, E>;
