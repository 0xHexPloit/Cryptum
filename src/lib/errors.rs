use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CryptumError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    #[error("Hex parsing error: {0}")]
    HexParser(#[from] hex::FromHexError),
    #[error("The length of the message is exceeding the authorized limit")]
    MessageLength,
    #[error("An unknown error as occurred")]
    Unknown,
}