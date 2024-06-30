use std::num::ParseIntError;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Chapter parse error: {0}")]
    ParseChapterError(#[from] ParseIntError),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Unexpected error occured: {0}")]
    UnexpetedError(String),
}

pub type Result<T> = std::result::Result<T, Error>;
