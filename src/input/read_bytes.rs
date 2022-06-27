use std::fs::File;
use std::io::{self, Read};

use crate::util::error_messages::ERROR_OPEN_INPUT_FILE_FAILED;
use crate::util::exit_codes::EXIT_OPEN_INPUT_FILE_FAILED;
use crate::util::global_constants::{MAX_BUFFERSIZE, MIN_BUFFERSIZE, ONE_GIGABYTE};


pub fn get_specific_number_of_bytes(path: &str, byte_count: usize) -> Vec<u8> {
    let bytes = match read_specific_number_of_bytes_to_vector(path, byte_count) {
        Ok(b) => b,
        Err(_) => Vec::new(),
    };

    bytes
}


fn read_specific_number_of_bytes_to_vector(path: &str, byte_count: usize) -> io::Result<Vec<u8>> {
    let mut file = get_input_file(&String::from(path));
    let mut buffer: Vec<u8> = vec![0; byte_count];

    file.read_exact(&mut buffer)?;

    Ok(buffer)
}

pub fn get_input_file(file: &String) -> File {
    let file = match File::open(file) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{}: {}", ERROR_OPEN_INPUT_FILE_FAILED, e);
            std::process::exit(EXIT_OPEN_INPUT_FILE_FAILED);
        }
    };

    file
}

pub fn calculate_capacity(buffersize: usize) -> usize {
    let mut buf_size = buffersize;
    if buf_size > MAX_BUFFERSIZE {
        buf_size = MAX_BUFFERSIZE;
    } else if buf_size < MIN_BUFFERSIZE {
        buf_size = MIN_BUFFERSIZE;
    }

    ONE_GIGABYTE * buf_size
}

#[test]
fn get_specific_bytes_smaller_than_file() {
    let path: &str = "tests/inputs/test_small.bin";
    assert_eq!(get_specific_number_of_bytes(path, 300).len(), 300);
}

#[test]
fn get_specific_bytes_equal_than_file() {
    let path: &str = "tests/inputs/test_small.bin";
    assert_eq!(get_specific_number_of_bytes(path, 357).len(), 357);
}

#[test]
fn get_specific_bytes_bigger_than_file() {
    let path: &str = "tests/inputs/test_small.bin";
    assert_eq!(get_specific_number_of_bytes(path, 500).len(), 0);
}

#[test]
fn get_specific_bytes_0() {
    let path: &str = "tests/inputs/test_small.bin";
    assert_eq!(get_specific_number_of_bytes(path, 0).len(), 0);
}
