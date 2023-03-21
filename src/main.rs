use structopt::StructOpt;
use cryptum::{CryptumArgs, CryptumResult};
use cryptum::handler::kyber_handler;

fn main() -> CryptumResult<()> {
    let args = CryptumArgs::from_args();

    match args {
        CryptumArgs::KYBER(args) => {
            kyber_handler(args)
        }
    }
}
