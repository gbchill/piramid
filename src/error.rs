use std::io;
use thiserror::Error;

// shorthand so we don't have to write Result<T, PiramidError> everywhere
pub type Result<T> = std::result::Result<T, PiramidError>;

// all the errors that can happen in our database
#[derive(Error, Debug)]
pub enum PiramidError {
    // file reading/writing failed
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    // couldn't serialize/deserialize data
    #[error("Serialization error: {0}")]
    Serialization(#[from] bincode::Error),
}
