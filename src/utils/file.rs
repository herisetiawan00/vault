use std::{fs, path::Path, process};

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
    let file_bytes = match fs::read(&path) {
        Ok(data) => data,
        Err(err) => {
            println!("Failed to read file {}: {}", &path, err);
            process::exit(1);
        }
    };
    bytes_to_bits(file_bytes)
}

pub fn get_file_stem_name(full_path: String) -> String {
    let path = Path::new(&full_path);

    if let Some(file_stem) = path.file_stem() {
        if let Some(file_stem_str) = file_stem.to_str() {
            return file_stem_str.to_string();
        } else {
            println!("File name is not valid UTF-8");
        }
    } else {
        println!("No file name found in path");
    }
    process::exit(1)
}
