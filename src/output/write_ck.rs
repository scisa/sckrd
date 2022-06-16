use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::sync::{Mutex, Arc};

use crate::util::error_messages::*;
use crate::util::exit_codes::*;
use crate::util::global_constants::OUTPUT_FILE_PATH;

pub struct WriteOptions {
    pub is_output_file: bool,
    pub is_basic_output: bool,
    pub is_verbose: bool,
    pub is_suppress_output: bool,
}

impl WriteOptions {
    pub fn new(is_output_file: bool, is_basic_output: bool, is_verbose: bool, is_suppress_output: bool) -> Self {
        Self {
            is_output_file: is_output_file,
            is_basic_output: is_basic_output,
            is_verbose: is_verbose,
            is_suppress_output: is_suppress_output,
        }
    }
}

pub fn write(crypto_key: &str, entropy: f32, key_length_byte: usize, write_options: &WriteOptions, file: Arc<Mutex<File>>) {
    if !write_options.is_suppress_output {
        write_to_stdout(crypto_key, entropy, key_length_byte, &write_options);
    }
    
    if write_options.is_output_file {
        write_to_file(crypto_key, entropy, file)
    }
}

fn write_to_stdout(
    crypto_key: &str,
    entropy: f32,
    key_length_byte: usize,
    write_options: &WriteOptions,
) {
    if write_options.is_basic_output {
        println!("{}", crypto_key);
    } else {
        if write_options.is_verbose {
            println!(
                "Possible Key: {}\t\tEntropy: {}\t\tSize (Byte): {}",
                crypto_key, entropy, key_length_byte
            );
        } else {
            println!("{}: {}", crypto_key, entropy);
        }
    }
}

pub fn get_file() -> File {
    let file = match OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(OUTPUT_FILE_PATH) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("{}: {}", ERROR_OUTPUT_FILE_CAN_NOT_BE_CREATED, e);
                std::process::exit(EXIT_OUTPUT_FILE_CAN_NOT_BE_CREATED);
            }
        };

    file
}

fn write_to_file(crypto_key: &str, entropy: f32, file: Arc<Mutex<File>>) {  
    let mut file_guard = file.lock().unwrap();
    let data = format!("{}: {}\n", crypto_key, entropy);
    if let Err(e) = file_guard.write_all(data.as_bytes()) {
        eprintln!("{}: {}", ERROR_OUTPUT_FILE_IS_NOT_WRITEABLE, e);
    }
}

pub fn remove_output_file(is_output_file: bool) {
    if is_output_file {
        if Path::new(OUTPUT_FILE_PATH).exists() {
            match fs::remove_file(OUTPUT_FILE_PATH) {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("{}: {}", ERROR_EXISTING_OUTPUT_FILE_CAN_NOT_BE_REMOVED, e);
                    std::process::exit(EXIT_EXISTING_OUTPUT_FILE_CANNOT_BE_REMOVED);
                }
            };
        }
    }
    
}

