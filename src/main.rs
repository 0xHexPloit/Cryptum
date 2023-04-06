use structopt::StructOpt;
use cryptum::{CryptumArgs, CryptumResult};
use cryptum::handler::kyber_handler;

fn main() -> CryptumResult<()> {
    let args = CryptumArgs::from_args();

    match args {
        CryptumArgs::KYBER(args) => {
            let result = kyber_handler(args);
            if let Err(e) = result {
                println!("{}", e.to_string())
            }
        }
    }

    Ok(())
}
