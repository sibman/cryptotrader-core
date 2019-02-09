#![allow(dead_code)]
#![allow(unused_variables)]

use std::{ fmt, fmt::Display, error::Error };

// pub type CoreResult<T> = Result<T, TrailerError>;
pub type CoreResult<T> = Result<T,Box<std::error::Error>>;

#[derive(Debug)]
pub enum TrailerError {
    ImportError,
    CommandError,
    ConfigError(String),
    Unsupported,
    Generic(String),
    APIError(String),
    MissingArgumentError(String),
}

impl Error for TrailerError {
    fn description(&self) -> &str { "TrailerError" }
}

impl Display for TrailerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            TrailerError::ImportError => write!(f, "ImportError"),
            TrailerError::APIError(err) => write!(f, "APIError: {}", err),
            TrailerError::CommandError => write!(f, "CommandError"),
            TrailerError::ConfigError(err) => write!(f, "Config error"),
            TrailerError::Unsupported => write!(f, "Unsupported"),
            TrailerError::Generic(err) => write!(f, "Generic: {}", err),
            TrailerError::MissingArgumentError(err) => write!(f, "MissingArgumentError: {}", err),
        }
    }
}
