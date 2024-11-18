use std::collections::BTreeMap;

use crate::{
    common::constants,
    utils::{
        bits::bits_to_bytes,
        encrypt::encryptor,
        file::{read_file_as_bits, write_to_file_recursively},
    },
};

pub fn encrypt_file(args: BTreeMap<String, String>) -> () {
    let file_path = args.get("file").unwrap();

    let file_bits = read_file_as_bits(file_path.to_string());

    let (encrypted, keys) = encryptor(file_bits);
    let encrypted_bytes: Vec<u8> = bits_to_bytes(encrypted.to_vec());
    let keys_bytes = bits_to_bytes(keys.to_vec());

    let base_directory = constants::get_base_directory();

    let encrypted_file_path = format!("{}/.vault/default/encrypted.bzf", base_directory);
    let encrypted_keys_path = format!("{}/.vault/default/encrypted.bzk", base_directory);

    write_to_file_recursively(&encrypted_file_path, &encrypted_bytes).unwrap();
    write_to_file_recursively(&encrypted_keys_path, &keys_bytes).unwrap();
}
