use crate::algorithms::byte_array::ByteArray;
use crate::algorithms::kyber::{KyberKEM512, KyberKEM768, KyberKEM1024, KyberKEM, KyberPKE, KyberCPAPKE512, KyberCPAPKE768, KyberCPAPKE1024, get_random_coin, KYBER_MESSAGE_LENGTH};
use crate::cli::kyber::{KyberArgs, KyberKEMArgs, KyberKEMDecryptArgs, KyberKEMEncryptArgs, KyberKeyGenArgs, KyberPKEArgs, KyberPKEDecryptArgs, KyberPKEEncryptArgs};
use crate::CryptumResult;
use crate::handler::utils::{read_data_from_file, write_data_to_disk};

pub fn get_pke_kyber(spec: u16) -> Box<dyn KyberPKE> {
    match spec {
        512 => {
            Box::new(KyberCPAPKE512::init())
        },
        768 => {
            Box::new(KyberCPAPKE768::init())
        },
        1024 => {
            Box::new(KyberCPAPKE1024::init())
        },
        _ => {
            panic!("Received invalid value for the Kyber PKE version to be used")
        }
    }
}

pub fn kyber_pke_keygen(args: KyberKeyGenArgs) -> CryptumResult<()> {
    let kyber = get_pke_kyber(args.spec);
    let seed = ByteArray::random(32);
    let (public_key, private_key) = kyber.keygen(seed);

    write_data_to_disk(public_key.to_hex(), args.out_pubkey)?;
    write_data_to_disk(private_key.to_hex(), args.out_privkey)?;

    Ok(())
}

pub fn kyber_pke_encrypt(args: KyberPKEEncryptArgs) -> CryptumResult<()> {
    let kyber = get_pke_kyber(args.spec);

    let plaintext_raw = read_data_from_file(args.in_plaintext)?;
    let plaintext = ByteArray::from(plaintext_raw.into_bytes());

    let public_key_raw = read_data_from_file(args.in_pubkey)?;
    let public_key = ByteArray::from_hex(public_key_raw)?;

    let mut cipher_text_str = String::new();

    for chunk in plaintext.get_bytes().chunks(KYBER_MESSAGE_LENGTH) {
        let chunk_length = chunk.len();
        let mut chunk_vec = chunk.to_vec();

        if chunk_length != KYBER_MESSAGE_LENGTH {
            chunk_vec.extend(vec![0; KYBER_MESSAGE_LENGTH - chunk_length])
        }

        let random_coin = get_random_coin();

        let ciphertext = kyber.encrypt(
            public_key.clone(),
            chunk_vec.into(),
            random_coin
        );

        cipher_text_str.push_str(ciphertext.to_hex().as_str());
    }

    if args.out_ciphertext.is_none() {
        print!("{}", cipher_text_str);
    } else {
        write_data_to_disk(cipher_text_str, args.out_ciphertext.unwrap())?;
    }

    Ok(())
}

pub fn kyber_pke_decrypt(args: KyberPKEDecryptArgs) -> CryptumResult<()> {
    let kyber = get_pke_kyber(args.spec);
    let ciphertext_length = kyber.get_ciphertext_length();

    let ciphertext_raw = read_data_from_file(args.in_ciphertext)?;
    let ciphertext = ByteArray::from_hex(ciphertext_raw)?;

    let private_key_raw = read_data_from_file(args.in_privkey)?;
    let private_key = ByteArray::from_hex(private_key_raw)?;

    let mut plaintext_str = String::new();

    for chunk in ciphertext.get_bytes().chunks(ciphertext_length) {
        let chunk_length = chunk.len();
        let mut chunk_vec = chunk.to_vec();

        if chunk_length != ciphertext_length {
            chunk_vec.extend(vec![0; ciphertext_length - chunk_length])
        }

        let plaintext = kyber.decrypt(
            private_key.clone(),
            chunk_vec.into()
        );

        let plaintext_bytes: Vec<u8> = plaintext.get_bytes().to_vec().iter().map(|&val| val.clone()).filter(|&val| val != 0).collect();
        plaintext_str.push_str(String::from_utf8_lossy(plaintext_bytes.as_slice()).as_ref());
    }

    if args.out_plaintext.is_none() {
        println!("{}", plaintext_str)
    } else {
        write_data_to_disk(plaintext_str, args.out_plaintext.unwrap())?;
    }

    Ok(())
}


pub fn kyber_pke_handler(args: KyberPKEArgs) -> CryptumResult<()> {
    match args {
        KyberPKEArgs::KEYGEN(args) => {
            kyber_pke_keygen(args)
        },
        KyberPKEArgs::ENCRYPT(args) => {
            kyber_pke_encrypt(args)
        },
        KyberPKEArgs::DECRYPT(args) => {
            kyber_pke_decrypt(args)
        }
    }
}

pub fn get_kem_kyber(spec: u16) -> Box<dyn KyberKEM> {
    match spec {
        512 => {
            Box::new(KyberKEM512::init())
        },
        768 => {
           Box::new( KyberKEM768::init())
        },
        1024 => {
            Box::new(KyberKEM1024::init())
        }
        _ => {
            panic!("Received invalid value for the Kyber KEM version to be used");
        }
    }
}


pub fn kyber_kem_keygen(args: KyberKeyGenArgs) -> CryptumResult<()> {
    let kyber = get_kem_kyber(args.spec);
    let seed = ByteArray::random(64);
    let (public_key, private_key) = kyber.keygen(seed);

    write_data_to_disk(public_key.to_hex(), args.out_pubkey)?;
    write_data_to_disk(private_key.to_hex(), args.out_privkey)?;

    Ok(())
}


pub fn kyber_kem_encrypt(args: KyberKEMEncryptArgs) -> CryptumResult<()> {
    let kyber = get_kem_kyber(args.spec);
    let pub_key_hex = read_data_from_file(args.in_pubkey)?;
    let pub_key = ByteArray::from_hex(pub_key_hex)?;

    let seed = ByteArray::random(32);

    let (ciphertext, shared_key) = kyber.encrypt(
        pub_key,
        seed,
        args.key_size
    );

    write_data_to_disk(ciphertext.to_hex(), args.out_ciphertext)?;
    write_data_to_disk(shared_key.to_hex(), args.out_shared)?;

    Ok(())
}

pub fn kyber_kem_decrypt(args: KyberKEMDecryptArgs) -> CryptumResult<()> {
    let kyber = get_kem_kyber(args.spec);
    let ciphertext_hex = read_data_from_file(args.in_ciphertext)?;
    let ciphertext = ByteArray::from_hex(ciphertext_hex)?;

    let priv_key_raw = read_data_from_file(args.in_privkey)?;
    let priv_key = ByteArray::from_hex(priv_key_raw)?;

    let shared_key = kyber.decrypt(
        ciphertext,
        priv_key,
        args.key_size
    );

    if args.out_shared.is_none() {
        println!("{}", shared_key.to_hex())
    } else {
        write_data_to_disk(shared_key.to_hex(), args.out_shared.unwrap())?;
    }
    Ok(())
}


pub fn kyber_kem_handler(args: KyberKEMArgs) -> CryptumResult<()> {
    match args {
        KyberKEMArgs::KEYGEN(args ) => {
            kyber_kem_keygen(args)
        },
        KyberKEMArgs::ENCRYPT(args) => {
            kyber_kem_encrypt(args)
        },
        KyberKEMArgs::DECRYPT(args) => {
            kyber_kem_decrypt(args)
        }
    }
}


pub fn kyber_handler(args: KyberArgs) -> CryptumResult<()> {
    match args {
        KyberArgs::KEM(args) => {
            kyber_kem_handler(args)
        },
        KyberArgs::PKE(args) => {
            kyber_pke_handler(args)
        }
    }
}