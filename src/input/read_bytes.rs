use std::fs::File;
use std::io::{self, BufReader, Read};

use crate::util::exit_codes::*;

pub fn get_bytes(path: &str, byte_count: usize) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();
    if byte_count != 0 {
        bytes = match read_specific_number_of_bytes_to_vector(path, byte_count) {
            Ok(b) => b,
            Err(_) => Vec::new(),
        };
    }

    if bytes.len() == 0 {
        bytes = match read_bytes_to_vector(&path) {
            Ok(b) => b,
            Err(_) => std::process::exit(EXIT_READ_BYTES_TO_VECTOR_FAILED),
        };
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

fn read_specific_number_of_bytes_to_vector(path: &str, byte_count: usize) -> io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut buffer: Vec<u8> = vec![0; byte_count];

    file.read_exact(&mut buffer)?;

    Ok(buffer)
}
