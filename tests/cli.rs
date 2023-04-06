use assert_cmd::Command;
use std::error::Error;
use std::fs;
use rand::distributions::Alphanumeric;
use rand::{Rng, thread_rng};

type TestResult = Result<(), Box<dyn Error>>;

const PRG: &str = "cryptum";
const PLAINTEXT: &str = "tests/inputs/plaintext.txt";
const PKE_PUB_KEY: &str = "tests/inputs/kyber_pke.pub";
const PKE_PRIV_KEY: &str = "tests/inputs/kyber_pke.priv";
const KEM_PUB_KEY: &str = "tests/inputs/kyber_kem.pub";
const KEM_PRIV_KEY: &str = "tests/inputs/kyber_kem.priv";

const DEFAULT_KEY_SIZE: usize = 32;

fn generate_test_file_path() -> String {
    loop {
        let filename: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        if fs::metadata(&filename).is_err() {
            return format!("tests/{}", filename);
        }
    }
}


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

fn encrypt_data(out_ciphertext: &str) -> TestResult {
    let args = &[
        "kyber",
        "pke",
        "encrypt",
        "--in-plaintext",
        PLAINTEXT,
        "--in-pubkey",
        PKE_PUB_KEY,
        "--out-ciphertext",
        out_ciphertext
    ];

    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .success();
    Ok(())
}


#[test]
fn test_pke_encrypt() -> TestResult {
    let out_ciphertext = generate_test_file_path();
    encrypt_data(out_ciphertext.as_str())?;

    let ciphertext = fs::read_to_string(out_ciphertext.clone())?;

    assert_ne!(ciphertext.len(), 0);

    fs::remove_file(out_ciphertext)?;

    Ok(())
}


#[test]
fn test_pke_decrypt() -> TestResult {
    let out_ciphertext = generate_test_file_path();

    encrypt_data(out_ciphertext.as_str())?;
    let args = &[
        "kyber",
        "pke",
        "decrypt",
        "--in-ciphertext",
        out_ciphertext.as_str(),
        "--in-privkey",
        PKE_PRIV_KEY
    ];

    let original_plaintext = fs::read_to_string(PLAINTEXT)?;

    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .success()
        .stdout(predicates::str::contains(original_plaintext));

    fs::remove_file(out_ciphertext)?;

    Ok(())
}

fn generate_kem_ciphertext_shared_key(
    key_size: usize,
    out_ciphertext_path: &str,
    out_shared_key_path: &str
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
        out_ciphertext_path,
        "--out-shared",
        out_shared_key_path
    ];
    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .success();
    Ok(())
}

#[test]
fn test_kem_keygen() -> TestResult {
    let out_pubkey_path = generate_test_file_path();
    let out_privkey_path = generate_test_file_path();

    let args = &[
        "kyber",
        "kem",
        "keygen",
        "--out-pubkey",
        out_pubkey_path.as_str(),
        "--out-privkey",
        out_privkey_path.as_str()
    ];

    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .success();


    let pubkey = fs::read_to_string(out_pubkey_path.clone())?;
    assert_ne!(pubkey.len(), 0);

    let privkey = fs::read_to_string(out_pubkey_path.clone())?;
    assert_ne!(privkey.len(), 0);

    fs::remove_file(out_pubkey_path)?;
    fs::remove_file(out_privkey_path)?;

    Ok(())
}


#[test]
fn test_kem_encrypt() -> TestResult {
    let out_ciphertext_path = generate_test_file_path();
    let out_shared_key_path = generate_test_file_path();


    generate_kem_ciphertext_shared_key(
        DEFAULT_KEY_SIZE,
        out_ciphertext_path.as_str(),
        out_shared_key_path.as_str()
    )?;

    let ciphertext = fs::read_to_string(out_ciphertext_path.clone())?;
    assert_ne!(ciphertext.len(), 0);

    let shared_key = fs::read_to_string(out_shared_key_path.clone())?;
    assert_ne!(shared_key.len(), 0);

    fs::remove_file(out_ciphertext_path)?;
    fs::remove_file(out_shared_key_path)?;

    Ok(())
}

#[test]
fn test_kem_decrypt() -> TestResult {
    let out_ciphertext_path = generate_test_file_path();
    let out_shared_key_path = generate_test_file_path();

    generate_kem_ciphertext_shared_key(
        DEFAULT_KEY_SIZE,
        out_ciphertext_path.as_str(),
        out_shared_key_path.as_str()
    )?;
    let keysize = format!("{}", DEFAULT_KEY_SIZE);
    let shared_key = fs::read_to_string(out_shared_key_path.clone())?;

    let args = &[
        "kyber",
        "kem",
        "decrypt",
        "--in-ciphertext",
        out_ciphertext_path.as_str(),
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

    fs::remove_file(out_ciphertext_path)?;
    fs::remove_file(out_shared_key_path)?;

    Ok(())
}

#[test]
fn test_generate_shared_key_different_size() -> TestResult {
    let out_ciphertext_path = generate_test_file_path();
    let out_shared_key_path = generate_test_file_path();

    generate_kem_ciphertext_shared_key(
        DEFAULT_KEY_SIZE * 2,
        out_ciphertext_path.as_str(),
        out_shared_key_path.as_str()
    )?;

    let shared_key = fs::read_to_string(out_shared_key_path.clone())?;
    assert_eq!(shared_key.len(), DEFAULT_KEY_SIZE * 2 * 2);

    fs::remove_file(out_ciphertext_path)?;
    fs::remove_file(out_shared_key_path)?;

    Ok(())
}

