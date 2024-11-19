## Introduction
Vault is custom file encryption system that works by moving bit per bit of the file.
Vault is using symmetric encryption, so it will generate pair of encrypted file and keys and you can also secure it by giving your custom passkey.

## Features
* Simple and secure encryption method
* Can be use in CI/CD
* Multi security layer (key and passkey)

## Concept
![vault-concept](https://github.com/herisetiawan00/vault/blob/main/doc/assets/vault-concept.png)


## Usages
```bash
vault [command] [...params]
```
| command                                                                                            | alias | description                                                                        |
|----------------------------------------------------------------------------------------------------|-------|------------------------------------------------------------------------------------|
| encrypt --file <file> -output-path <output-path>                                                   | e     | Encrypt given file and return encrypted file and key                               |
| decrypt --file <encrypted-file> -key <key-file> [ --output <output-file> ] [ --passkey <passkey> ] | d     | Decrypt given file with the key, will print the content of `--output` not provided |

## Dependencies
* Rust (cargo)

## Installation
* install (rustup)[https://doc.rust-lang.org/cargo/getting-started/installation.html]
* clone this repo
```bash
git clone https://github.com/herisetiawan00/vault.git
```
* cd to the directory
```bash
cd vault
```
* build source code
```bash
make build 
```
* if you are using mac or linux you can move binary to `/usr/bin` folder by using
```bash
make install
```

## Uninstall
* Simply remove the binary by using
```bash
make uninstall
```

## Contribution
Feel free to contribute and creating the pull request. It will be really helpful :D

