mod utils;
use std::env;

use utils::bits::{bits_to_string, string_to_bits};
use utils::encrypt::{decryptor, encryptor};

fn main() {
    let _args: Vec<String> = env::args().collect();

    let input: &str = "Lorem Ipsum Dolor Sit Amet";
    let bit_list = string_to_bits(input);

    match bits_to_string(&bit_list) {
        Ok(result) => println!("{}", result),
        Err(err) => println!("{}", err),
    }

    let (encrypted, keys) = encryptor(&bit_list);

    let decrypted = decryptor(&encrypted, &keys);
    let output = bits_to_string(&decrypted);

    match output {
        Ok(result) => println!("{}", result),
        Err(err) => println!("{}", err),
    }
}
