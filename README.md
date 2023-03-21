# Cryptum

This project consists of a CLI program written in Rust that allows either encrypting or signing data using lattice-based cryptography. For the encryption part, the `Kyber` algorithm ([article](https://eprint.iacr.org/2017/634.pdf)) has been used. For the signature algorithm, we opted for the `Dilithium` algorithm ([article](https://eprint.iacr.org/2017/633.pdf)).


# Usage

Until the creation of an x86_64 binary, you can interact with the program using the following commands:

```
cargo run -- kyber kem keygen
```

```
cargo run -- kyber kem encrypt
```

```
cargo run -- kyber kem decrypt
```

ℹ️ Normally you will not have to change the default values specified for each command. However, if you would like to change some of them, don't hesitate to add the `--help` at the end of each command as illustrated below:
``
cargo run -- kyber kem decrypt --help
``

Using this command should output something similar to the figure above

![help](https://github.com/0xHexPloit/Cryptum/blob/main/assets/encrypt_help_output.png?raw=true)
