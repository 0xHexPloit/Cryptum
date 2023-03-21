use crate::algorithms::byte_array::ByteArray;
use crate::algorithms::kyber::{Kyber1024, Kyber512, Kyber768, KyberAlgorithm};
use crate::cli::CryptumResult;
use crate::cli::kyber::{KyberArgs, KyberKEMArgs, KyberKEMKeyGenArgs};
use crate::handler::utils::write_data_to_disk;


pub fn get_kem_kyber  (spec: u16) -> Box<dyn KyberAlgorithm> {
    match spec {
        512 => {
            Box::new(Kyber512::init())
        },
        768 => {
           Box::new( Kyber768::init())
        },
        1024 => {
            Box::new(Kyber1024::init())
        }
        _ => {
            panic!("Received invalid value for the version of kyber to use");
        }
    }
}


pub fn kyber_kem_keygen(args: KyberKEMKeyGenArgs) -> CryptumResult<()> {
    let kyber = get_kem_kyber(args.spec);
    let seed = ByteArray::random(64);
    let (public_key, private_key) = kyber.keygen(seed);

    write_data_to_disk(public_key.to_hex(), args.out_pubkey)?;
    write_data_to_disk(private_key.to_hex(), args.out_privkey)?;

    Ok(())
}


pub fn kyber_kem_handler(args: KyberKEMArgs) -> CryptumResult<()> {
    match args {
        KyberKEMArgs::KEYGEN(args ) => {
            kyber_kem_keygen(args)
        }
    }
}


pub fn kyber_handler(args: KyberArgs) -> CryptumResult<()> {
    match args {
        KyberArgs::KEM(args) => {
            kyber_kem_handler(args)
        }
    }
}