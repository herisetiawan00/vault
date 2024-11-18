use std::{fs, path::Path};

use super::bits::bytes_to_bits;

pub fn write_to_file_recursively(path: &str, contents: &[u8]) -> std::io::Result<()> {
    let path = Path::new(path);

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(path, contents)?;

    Ok(())
}

pub fn read_file_as_bits(path: String) -> Vec<u8> {
    let file_bytes = fs::read(path).unwrap();
    bytes_to_bits(file_bytes)
}
