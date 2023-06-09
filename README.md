![badge](https://github.com/0xHexPloit/Cryptum/actions/workflows/rust.yml/badge.svg)

# Cryptum

**WARNING: This project was developed as part of a cryptographic project for the advanced cryptography course given at Télécom Paris. It is therefore not intended to be used in production, in particular because it has not been tested to determine whether it runs in constant time.**

This project consists of an implementation of [Kyber](https://pq-crystals.org/kyber/data/kyber-specification-round3-20210804.pdf) post-quantum public-key encryption and key-establishment algorithm as well as an implementation of [Dilithium](https://pq-crystals.org/dilithium/data/dilithium-specification-round3-20210208.pdf) post-quantum secure digital signature algorithm (not yet implemented).

 Kyber is one of the NIST post-quantum cryptography (PQC) standardization initiative's selected algorithms, offering both IND-CPA-secure public key encryption and IND-CCA2-secure key encapsulation mechanism.

Concerning Dilithium, the latter is designed to be secure against attacks from quantum computers, which are expected to break many of the traditional digital signature algorithms used today. It provides strong security guarantees while also being efficient enough to be practical for real-world use cases. To be more precise, Dilithium can be used for a variety of applications where digital signatures are required, including secure communication protocols, software updates, and digital transactions.

## Motivation

The security of Kyber is based on the hardness of solving learning-with-errors (LWE) problem in module lattices. Kyber provides secure communication solutions for both asynchronous (e.g., email) and synchronous (e.g., TLS) settings.

Similar to Kyber, the Dilithium's security is based on the hardness of solving certain mathematical problems, in this case related to ideal lattices.

## Kyber

### Kyber CPAPKE

Under IND-CPA-secure Kyber PKE, two communicating parties generate their key pairs and publish their public keys to each other. Using the recipient's public key, a sender can encrypt a fixed-length message (32-bytes) using a random coin (32-bytes) as input to the encryption algorithm. The ciphertext can be decrypted by the recipient's secret key, which is private to the key owner, to recover the original message.

#### Algorithm
Algorithm | Input | Output
--- | :-: | --:
PKE KeyGen | - | Public Key and Secret Key
Encapsulation | Public Key, 32-bytes message, and 32-bytes random coin| Cipher Text
Decapsulation | Secret Key and Cipher Text | 32-bytes message

Each algorithm can be run using the CLI program with the following commands
```
./cryptum kyber pke keygen
```

```
./cryptum kyber pke encrypt -in-plaintext <plaintext> -out-ciphertext <ciphertext>
```
```
./cryptum kyber pke decrypt --in-ciphertext <ciphertext>
```

### Kyber CCAKEM

Kyber CCAKEM is an IND-CCA2-secure KEM constructed by applying a slightly tweaked Fujisaki–Okamoto (FO) transform on IND-CPA-secure Kyber PKE. In this mechanism, two parties interested in secretly communicating over a public and insecure channel generate a shared secret key of arbitrary byte length from a key derivation function (KDF) obtained by seeding SHAKE256 XOF with the same secret. The secret key is 32-bytes (by default), and the sender communicates it to the recipient using the underlying Kyber PKE.

#### Algorithm
Algorithm | Input | Output
--- | :-: | --:
KEM KeyGen | - | Public Key and Secret Key
Encapsulation | Public Key | Cipher Text and SHAKE256 KDF
Decapsulation | Secret Key and Cipher Text | SHAKE256 KDF

Each algorithm can be run using the CLI program with the following commands
```
./cryptum kyber kem keygen 
```
```
./cryptum kyber kem encrypt
```

```
./cryptum kyber kem decrypt
```
ℹ️ For local usage, we suggest to add the `--out-shared` option so as to avoid overriding the content of the file generated by the `encrypt` command.

## Notes

Normally you will not have to change the default values specified for each command. However, if you would like to change some of them, don't hesitate to add the `--help` at the end of each command as illustrated below:
```
./cryptum kyber pke decrypt --help
```

Using this command should output something similar to the figure above

![help](https://github.com/0xHexPloit/Cryptum/blob/main/assets/decrypt_help.png?raw=true)


