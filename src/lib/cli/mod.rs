
use structopt::{StructOpt};
pub mod kyber;
mod results;

#[derive(StructOpt, Debug)]
#[structopt(
    author = "Hugo PEYRON",
    version = "0.1.0",
    about = "A CLI program that allows either encrypting or signing data using lattice-based cryptography."
)]
pub enum CryptumArgs {
    KYBER(kyber::KyberArgs)
}