use thiserror::Error;

#[derive(Error, Debug)]
pub enum CryptumError {
    #[error("I/O Error: {0}")]
    IO(String),
    #[error("Hex parsing error: {0}")]
    HexParser(#[from] hex::FromHexError),
    #[error("An unknown error as occurred")]
    Unknown,
}