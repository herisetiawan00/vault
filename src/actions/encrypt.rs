use std::{collections::BTreeMap, process};

use crate::utils::{
    bits::bits_to_bytes,
    encrypt::encryptor,
    file::{get_file_stem_name, read_file_as_bits, write_to_file_recursively},
};

pub fn encrypt_file(args: BTreeMap<String, String>) -> () {
    let file_path = match args.get("file") {
        Some(value) => value,
        None => {
            println!("Missing required parameter [--file]");
            process::exit(1)
        }
    };
    let file_name = get_file_stem_name(file_path.to_string());

    let output_path = args.get("output-path").map_or("", |v| v);

    let file_bits = read_file_as_bits(file_path.to_string());

    let (encrypted, keys) = encryptor(file_bits);
    let encrypted_bytes: Vec<u8> = bits_to_bytes(encrypted.to_vec());
    let keys_bytes = bits_to_bytes(keys.to_vec());

    let encrypted_file_path = format!("{}{}.bzf", output_path, file_name);
    let encrypted_keys_path = format!("{}{}.bzk", output_path, file_name);

    match write_to_file_recursively(&encrypted_file_path, &encrypted_bytes) {
        Ok(_) => println!("Encrypted file stored at {}", encrypted_file_path),
        Err(e) => {
            println!(
                "Failed to store encrypted file at {}: {}",
                encrypted_file_path, e
            );
            process::exit(1);
        }
    }
    match write_to_file_recursively(&encrypted_keys_path, &keys_bytes) {
        Ok(_) => println!("Key file stored at {}", encrypted_keys_path),
        Err(e) => {
            println!(
                "Key to store encrypted file at {}: {}",
                encrypted_keys_path, e
            );
            process::exit(1);
        }
    }
}
