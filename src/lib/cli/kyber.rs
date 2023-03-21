use std::path::PathBuf;
use std::str::FromStr;
use structopt::StructOpt;


#[derive(StructOpt, Debug)]
pub enum KyberArgs {
    KEM(KyberKEMArgs),
}



#[derive(StructOpt, Debug)]
pub enum KyberKEMArgs {
    KEYGEN(KyberKEMKeyGenArgs)
}


#[derive(StructOpt, Debug)]
pub struct KyberKEMKeyGenArgs {
    #[structopt(short, long, default_value="512")]
    /// The version of the algorithm to use (512/768/1024)
    pub spec: u16,

    /// The path where to save the generated private key
    #[structopt(long, default_value="kyber_kem_key.priv", parse(from_os_str))]
    pub out_privkey: PathBuf,

    /// The path where to save the generated public key
    #[structopt(long, default_value="kyber_kem_key.pub", parse(from_os_str))]
    pub out_pubkey: PathBuf
}


