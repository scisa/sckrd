use std::fs::File;
use std::io::{self, BufReader, Read};

use crate::util::exit_codes::EXIT_READ_BYTES_TO_VECTOR_FAILED;
use crate::util::error_messages::ERROR_READ_BYTES_TO_VECTOR_FAILED;

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
            Err(e) => {
                eprintln!("{}: {}", ERROR_READ_BYTES_TO_VECTOR_FAILED, e);
                std::process::exit(EXIT_READ_BYTES_TO_VECTOR_FAILED)
            },
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


#[test]
fn get_specific_bytes_smaller_than_file() {
    let path: &str = "tests/inputs/test_small.bin";
    assert_eq!(get_bytes(path, 300).len(), 300);
}

#[test]
fn get_specific_bytes_equal_than_file() {
    let path: &str = "tests/inputs/test_small.bin";
    assert_eq!(get_bytes(path, 357).len(), 357);
}

#[test]
fn get_specific_bytes_bigger_than_file() {
    let path: &str = "tests/inputs/test_small.bin";
    assert_eq!(get_bytes(path, 500).len(), 357);
}

#[test]
fn get_specific_bytes_0() {
    let path: &str = "tests/inputs/test_small.bin";
    assert_eq!(get_bytes(path, 0).len(), 357);
}