use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct ConfigurationReaderError {
    pub error_message: String,
}

impl Display for ConfigurationReaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error_message)
    }
}

impl Error for ConfigurationReaderError {}
