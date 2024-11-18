use std::process;

use super::key::key_generator;

pub fn encryptor(input: Vec<u8>) -> (Vec<u8>, Vec<u8>) {
    let key_length = 12 * 8;
    let keys = key_generator(key_length);
    let mut rest = input.to_vec();
    let mut result: Vec<u8> = vec![];

    while rest.len() > 0 {
        let index = rest.len() % keys.len();
        let position: u8 = keys[index];
        let value = rest.pop().unwrap_or(2);

        match position {
            0 => result.insert(0, value),
            1 => result.push(value),
            _ => {}
        }
    }

    (result, keys)
}

pub fn decryptor(input: Vec<u8>, keys: Vec<u8>) -> Vec<u8> {
    let mut rest = input.to_vec();
    let mut result: Vec<u8> = vec![];
    let mut index: usize = 1;

    while rest.len() > 0 {
        let position = keys[index];

        let value: Option<u8> = match position {
            1 => Some(match rest.pop() {
                Some(value) => value,
                None => {
                    println!("Invalid given input data!");
                    process::exit(1);
                }
            }),
            0 => Some(rest.remove(0)),
            _ => None,
        };

        match value {
            Some(value) => result.push(value),
            None => {
                println!("Invalid given input data!");
                process::exit(1);
            }
        }

        if index == keys.len() - 1 {
            index = 0;
        } else {
            index += 1;
        }
    }

    result
}
