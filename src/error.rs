use std::fmt;
use reqwest::Error as ReqwestError;

#[derive(Debug)]
pub enum TealdeerError {
    CacheError(String),
    ConfigError(String),
    UpdateError(String),
}

impl From<ReqwestError> for TealdeerError {
    fn from(err: ReqwestError) -> TealdeerError {
        TealdeerError::UpdateError(format!("HTTP error: {}", err.to_string()))
    }
}

impl fmt::Display for TealdeerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TealdeerError::CacheError(e) => write!(f, "CacheError: {}", e),
            TealdeerError::ConfigError(e) => write!(f, "ConfigError: {}", e),
            TealdeerError::UpdateError(e) => write!(f, "UpdateError: {}", e),
        }
    }
}
