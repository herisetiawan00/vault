pub fn string_to_bits(input: &str) -> Vec<u8> {
    input
        .bytes()
        .flat_map(|byte| (0..8).rev().map(move |i| ((byte >> i) & 1) as u8))
        .collect()
}

pub fn bits_to_string(bits: &[u8]) -> Result<String, String> {
    if bits.len() % 8 != 0 {
        return Err("Bit list length must be a multiple of 8.".to_string());
    }

    let chars: Vec<u8> = bits
        .chunks(8)
        .map(|chunk| chunk.iter().fold(0, |byte, &bit| (byte << 1) | bit))
        .collect();

    String::from_utf8(chars).map_err(|e| e.to_string())
}
