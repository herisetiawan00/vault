use std::{collections::BTreeMap, fs, process};

use crate::utils::{bits::bits_to_bytes, encrypt::decryptor, file::read_file_as_bits};

pub fn decrypt_file(args: BTreeMap<String, String>) -> () {
    let file_path = args.get("file").unwrap();
    let keys_path = args.get("key").unwrap();
    let output_path = args.get("output");

    let file_bits = read_file_as_bits(file_path.to_string());
    let keys_bits = read_file_as_bits(keys_path.to_string());

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
