pub fn string_to_bits(string: &str) -> Vec<u8> {
    string
        .bytes()
        .flat_map(|byte| (0..8).rev().map(move |i| ((byte >> i) & 1) as u8))
        .collect()
}

pub fn bits_to_string(bits: Vec<u8>) -> Result<String, String> {
    if bits.len() % 8 != 0 {
        return Err("Bit list length must be a multiple of 8.".to_string());
    }

    let chars: Vec<u8> = bits
        .chunks(8)
        .map(|chunk| chunk.iter().fold(0, |byte, &bit| (byte << 1) | bit))
        .collect();

    String::from_utf8(chars).map_err(|e| e.to_string())
}

pub fn bytes_to_bits(bytes: Vec<u8>) -> Vec<u8> {
    let mut bits: Vec<u8> = Vec::new();

    for byte in bytes {
        for i in (0..8).rev() {
            bits.push((byte >> i) & 1)
        }
    }

    bits
}

pub fn bits_to_bytes(bits: Vec<u8>) -> Vec<u8> {
    assert!(
        bits.iter().all(|&b| b == 0 || b == 1),
        "Bits must be 0 or 1"
    );

    bits.chunks(8)
        .map(|chunk| {
            chunk
                .iter()
                .rev()
                .enumerate()
                .fold(0u8, |byte, (i, &bit)| byte | (bit << i))
        })
        .collect()
}
