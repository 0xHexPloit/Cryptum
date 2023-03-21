use std::error::Error;
use structopt::{StructOpt};
pub mod kyber;

pub type CryptumResult<T> = Result<T, Box<dyn Error>>;

#[derive(StructOpt, Debug)]
#[structopt(
    author = "Hugo PEYRON",
    version = "0.1.0",
    about = "A CLI program that allows either encrypting or signing data using lattice-based cryptography."
)]
pub enum CryptumArgs {
    KYBER(kyber::KyberArgs)
}