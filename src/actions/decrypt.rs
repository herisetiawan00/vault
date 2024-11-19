use std::{collections::BTreeMap, fs, process};

use rpassword::read_password;

use crate::utils::{
    bits::{bits_to_bytes, bytes_to_bits},
    encrypt::decryptor,
    file::read_file_as_bits,
};

pub fn decrypt_file(args: BTreeMap<String, String>) -> () {
    let file_path = match args.get("file") {
        Some(value) => value,
        None => {
            println!("Missing required parameter [--file]");
            process::exit(1)
        }
    };
    let keys_path = match args.get("key") {
        Some(value) => value,
        None => {
            println!("Missing required parameter [--key]");
            process::exit(1)
        }
    };

    let output_path = args.get("output");

    let file_bits = read_file_as_bits(file_path.to_string());
    let mut keys_bits = read_file_as_bits(keys_path.to_string());

    let passkey: Vec<u8> = match args.get("passkey") {
        Some(value) => bytes_to_bits(value.as_bytes().to_vec()),
        None => {
            println!("Enter your passkey (press enter to make it empty)");
            match read_password() {
                Ok(passkey) => bytes_to_bits(passkey.as_bytes().to_vec()),
                Err(_) => Vec::new(),
            }
        }
    };

    match passkey.len() {
        0 => {}
        _ => {
            let decrypted_keys = decryptor(keys_bits, passkey);
            keys_bits = decrypted_keys;
        }
    };

    let decrypted_bits = decryptor(file_bits, keys_bits);
    let decrypted_bytes = bits_to_bytes(decrypted_bits);

    match output_path {
        Some(value) => {
            let _ = fs::write(value, decrypted_bytes);
        }
        None => {
            let value = std::str::from_utf8(&decrypted_bytes);
            match value {
                Ok(text) => println!("{}", text),
                Err(_) => {
                    println!("Error: Failed to decrypt file");
                    process::exit(1)
                }
            }
        }
    }
}
