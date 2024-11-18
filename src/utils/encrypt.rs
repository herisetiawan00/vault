use super::key::key_generator;

pub fn encryptor(input: Vec<u8>) -> (Vec<u8>, Vec<u8>) {
    let key_length = 12 * 8;
    let keys = key_generator(key_length);
    let mut rest = input.to_vec();
    let mut result: Vec<u8> = vec![];

    while rest.len() > 0 {
        let index = rest.len() % keys.len();
        let position: u8 = keys[index];
        let value = rest.pop().unwrap();

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

        match position {
            1 => result.push(rest.pop().unwrap()),
            0 => result.push(rest.remove(0)),
            _ => {}
        }

        if index == keys.len() - 1 {
            index = 0;
        } else {
            index += 1;
        }
    }

    result
}
