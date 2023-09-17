use std::string::FromUtf8Error;

use serde::Serialize;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Serialize, Error, Debug)]
pub enum Error {
    #[error("Invalid URL")]
    Url,
    #[error("Arklib error")]
    Arklib,
    #[error("Alreay exist")]
    LinkExist,
    #[error("IO error")]
    IO,
}

impl From<url::ParseError> for Error {
    fn from(_: url::ParseError) -> Self {
        Self::Url
    }
}

impl From<std::io::Error> for Error {
    fn from(_: std::io::Error) -> Self {
        Self::IO
    }
}

impl From<FromUtf8Error> for Error {
    fn from(_: FromUtf8Error) -> Self {
        Self::IO
    }
}
