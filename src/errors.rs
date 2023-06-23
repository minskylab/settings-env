use std::{fmt::Display, io};

use serde::de::Error as SerdeError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SettingsError {
    #[error("failed to parse environment variable")]
    Disconnect(#[from] io::Error),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader { expected: String, found: String },
    #[error("deserialization error: {0}")]
    Deserialization(String),
    #[error("unknown data store error")]
    Unknown,
}

impl SerdeError for SettingsError {
    fn custom<T: Display>(msg: T) -> Self {
        SettingsError::Deserialization(msg.to_string())
    }
}
