use std::fs::File;
use std::io::{self, BufReader, Read};

use crate::util::exit_codes::*;

pub fn get_bytes(path: &str, byte_count: usize) -> Vec<u8> {
    let mut bytes = match read_bytes_to_vector(&path) {
        Ok(b) => b,
        Err(_) => std::process::exit(EXIT_READ_BYTES_TO_VECTOR_FAILED),
    };

    if byte_count != 0 {
        if bytes.len() > byte_count {
            let _ = bytes.split_off(byte_count);
        }
    }

    bytes
}

fn read_bytes_to_vector(path: &str) -> io::Result<Vec<u8>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buffer: Vec<u8> = Vec::new();

    // Read file into vector.
    reader.read_to_end(&mut buffer)?;

    Ok(buffer)
}

