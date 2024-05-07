use thiserror::Error;

mod specific;

pub use specific::*;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    InvalidValue(#[from] InvalidValue),
    #[error(transparent)]
    PersistenceError(#[from] PersistenceError),
    #[error("{0}")]
    Forbiden(&'static str)
}

impl From<&'static str> for Error {
    fn from(value: &'static str) -> Self {
        Error::Forbiden(value)
    }
}


pub type Result<T> = core::result::Result<T, Error>;

pub fn error<T>(value: impl Into<Error>) -> Result<T> {
    Err(value.into())
}