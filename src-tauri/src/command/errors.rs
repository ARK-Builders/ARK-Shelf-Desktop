use std::string::FromUtf8Error;

use serde::Serialize;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, CommandError>;

#[derive(Serialize, Error, Debug)]
pub enum CommandError {
    #[error("Invalid URL")]
    Url,
    #[error("Arklib error")]
    Arklib,
    #[error("Alreay exist")]
    LinkExist,
    #[error("IO error")]
    IO,
}

impl From<url::ParseError> for CommandError {
    fn from(_: url::ParseError) -> Self {
        Self::Url
    }
}

impl From<std::io::Error> for CommandError {
    fn from(_: std::io::Error) -> Self {
        Self::IO
    }
}

impl From<FromUtf8Error> for CommandError {
    fn from(_: FromUtf8Error) -> Self {
        Self::IO
    }
}
