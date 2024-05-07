

use thiserror::Error;

#[derive(Debug, Error)]
#[error("Invalid value for {0}")]
pub struct InvalidValue(String);

impl InvalidValue {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }
}

#[derive(Debug, Error)]
#[error("Persistence error: {0}")]
pub struct PersistenceError(String);

impl PersistenceError {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }
}
