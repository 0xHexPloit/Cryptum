use std::path::PathBuf;
use structopt::StructOpt;


#[derive(StructOpt, Debug)]
pub enum KyberArgs {
    KEM(KyberKEMArgs),
}


#[derive(StructOpt, Debug)]
pub enum KyberKEMArgs {
    KEYGEN(KyberKEMKeyGenArgs),
    ENCRYPT(KyberKEMEncryptArgs),
    DECRYPT(KyberKEMDecryptArgs)
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


#[derive(StructOpt, Debug)]
pub struct KyberKEMEncryptArgs {
    #[structopt(short, long, default_value="512")]
    /// The version of the algorithm to use (512/768/1024)
    pub spec: u16,

    /// The path where to save the generated ciphertext
    #[structopt(long, default_value="kyber_ciphertext.txt", parse(from_os_str))]
    pub out_ciphertext: PathBuf,

    /// The path where to save the generated shared key
    #[structopt(long, default_value="kyber_kem_shared_key.txt", parse(from_os_str))]
    pub out_shared: PathBuf,

    /// The size of the shared key (in bytes)
    #[structopt(long, default_value="32")]
    pub key_size: u8,

    /// The path where is situated the public key
    #[structopt(long, default_value="kyber_kem_key.pub", parse(from_os_str))]
    pub in_pubkey: PathBuf
}


#[derive(StructOpt, Debug)]
pub struct KyberKEMDecryptArgs {
    #[structopt(short, long, default_value="512")]
    /// The version of the algorithm to use (512/768/1024)
    pub spec: u16,

    /// The path where to save the generated shared key
    #[structopt(long, default_value="kyber_kem_shared_key.txt")]
    pub out_shared: PathBuf,

    /// The size of the shared key (in bytes)
    #[structopt(long, default_value="32")]
    pub key_size: u8,

    /// The path where to load the ciphertext
    #[structopt(long, default_value="kyber_ciphertext.txt", parse(from_os_str))]
    pub in_ciphertext: PathBuf,

    /// The path where is situated the public key
    #[structopt(long, default_value="kyber_kem_key.priv", parse(from_os_str))]
    pub in_privkey: PathBuf
}

