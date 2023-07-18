use serde::Serialize;
use std::convert::{From};

#[derive(thiserror::Error, Serialize, Debug)]
pub enum Error {
    #[error("{0}")]
    Database(String),
    #[error("{0}")]
    Business(String),
    #[error("{0}")]
    System(String),
}

impl From<rbatis::rbdc::Error> for Error {
    fn from(err: rbatis::rbdc::Error) -> Self {
        Error::Database(err.to_string())
    }
}

impl From<fast_log::error::LogError> for Error {
    fn from(err: fast_log::error::LogError) -> Self {
        Error::System(err.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::System(err.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::System(err.to_string())
    }
}

