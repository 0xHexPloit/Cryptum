mod algorithms;
pub mod cli;
pub mod handler;
pub mod errors;
use crate::errors::CryptumError;

pub type CryptumResult<T> = Result<T, CryptumError>;

pub use cli::{CryptumArgs};
