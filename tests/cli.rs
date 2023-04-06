use assert_cmd::Command;
use std::error::Error;
use std::fs;

type TestResult = Result<(), Box<dyn Error>>;

const PRG: &str = "cryptum";
const PLAINTEXT: &str = "tests/inputs/plaintext.txt";
const PKE_PUB_KEY: &str = "tests/inputs/kyber_pke.pub";
const PKE_PRIV_KEY: &str = "tests/inputs/kyber_pke.priv";
const KEM_PUB_KEY: &str = "tests/inputs/kyber_kem.pub";
const KEM_PRIV_KEY: &str = "tests/inputs/kyber_kem.priv";

const OUT_CIPHERTEXT: &str = "tests/ciphertext.txt";
const OUT_SHARED_KEY: &str = "tests/shared_key.txt";

const DEFAULT_KEY_SIZE: usize = 32;


#[test]
fn usage() -> TestResult {
    for flag in &["-h", "--help"] {
        Command::cargo_bin(PRG)?
            .arg(flag)
            .assert()
            .success()
            .stdout(predicates::str::contains("USAGE"));
    }
    Ok(())
}

#[test]
fn test_pke_keygen() -> TestResult {
    let out_pubkey_path= "tests/kyber.pub";
    let out_privkey_path = "tests/kyber.priv";

    let args = &[
        "kyber",
        "pke",
        "keygen",
        "--out-pubkey",
        out_pubkey_path,
        "--out-privkey",
        out_privkey_path
    ];

    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .success();

    fs::read_to_string(out_pubkey_path)?;
    fs::read_to_string(out_privkey_path)?;

    fs::remove_file(out_pubkey_path)?;
    fs::remove_file(out_privkey_path)?;

    Ok(())
}

fn encrypt_data() -> TestResult {
    let args = &[
        "kyber",
        "pke",
        "encrypt",
        "--in-plaintext",
        PLAINTEXT,
        "--in-pubkey",
        PKE_PUB_KEY,
        "--out-ciphertext",
        OUT_CIPHERTEXT
    ];

    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .success();
    Ok(())
}


#[test]
fn test_pke_encrypt() -> TestResult {
    encrypt_data()?;
    let ciphertext = fs::read_to_string(OUT_CIPHERTEXT)?;

    assert_ne!(ciphertext.len(), 0);

    fs::remove_file(OUT_CIPHERTEXT)?;

    Ok(())
}


#[test]
fn test_pke_decrypt() -> TestResult {
    encrypt_data()?;
    let args = &[
        "kyber",
        "pke",
        "decrypt",
        "--in-ciphertext",
        OUT_CIPHERTEXT,
        "--in-privkey",
        PKE_PRIV_KEY
    ];

    let original_plaintext = fs::read_to_string(PLAINTEXT)?;

    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .success()
        .stdout(predicates::str::contains(original_plaintext));

    fs::remove_file(OUT_CIPHERTEXT)?;

    Ok(())
}

#[test]
fn test_kem_keygen() -> TestResult {
    let out_pubkey = "tests/kyber_kem.pub";
    let out_privkey = "tests/kyber_kem.priv";

    let args = &[
        "kyber",
        "kem",
        "keygen",
        "--out-pubkey",
        out_pubkey,
        "--out-privkey",
        out_privkey
    ];

    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .success();


    let pubkey = fs::read_to_string(out_pubkey)?;
    assert_ne!(pubkey.len(), 0);

    let privkey = fs::read_to_string(out_privkey)?;
    assert_ne!(privkey.len(), 0);

    fs::remove_file(out_pubkey)?;
    fs::remove_file(out_privkey)?;

    Ok(())
}


fn generate_kem_ciphertext_shared_key(
    key_size: usize
) -> TestResult {
    let key_size = format!("{}", key_size);
    let args = &[
        "kyber",
        "kem",
        "encrypt",
        "--in-pubkey",
        KEM_PUB_KEY,
        "--key-size",
        key_size.as_str(),
        "--out-ciphertext",
        OUT_CIPHERTEXT,
        "--out-shared",
        OUT_SHARED_KEY
    ];
    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .success();
    Ok(())
}

fn delete_kem_files() -> TestResult {
    fs::remove_file(OUT_CIPHERTEXT)?;
    fs::remove_file(OUT_SHARED_KEY)?;
    Ok(())
}


#[test]
fn test_kem_encrypt() -> TestResult {
    generate_kem_ciphertext_shared_key(DEFAULT_KEY_SIZE)?;

    let ciphertext = fs::read_to_string(OUT_CIPHERTEXT)?;
    assert_ne!(ciphertext.len(), 0);

    let shared_key = fs::read_to_string(OUT_SHARED_KEY)?;
    assert_ne!(shared_key.len(), 0);

    delete_kem_files()?;

    Ok(())
}

#[test]
fn test_kem_decrypt() -> TestResult {
    generate_kem_ciphertext_shared_key(DEFAULT_KEY_SIZE)?;
    let keysize = format!("{}", DEFAULT_KEY_SIZE);
    let shared_key = fs::read_to_string(OUT_SHARED_KEY)?;

    let args = &[
        "kyber",
        "kem",
        "decrypt",
        "--in-ciphertext",
        OUT_CIPHERTEXT,
        "--key-size",
        keysize.as_str(),
        "--in-privkey",
        KEM_PRIV_KEY
    ];
    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .success()
        .stdout(predicates::str::contains(shared_key));

    delete_kem_files()?;

    Ok(())
}

#[test]
fn test_generate_shared_key_different_size() -> TestResult {
    generate_kem_ciphertext_shared_key(DEFAULT_KEY_SIZE * 2)?;

    let shared_key = fs::read_to_string(OUT_SHARED_KEY)?;
    assert_eq!(shared_key.len(), DEFAULT_KEY_SIZE * 2 * 2);

    delete_kem_files()?;

    Ok(())
}

