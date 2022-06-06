use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::Path;

use crate::util::error_messages::*;
use crate::util::exit_codes::*;
use crate::util::global_constants::*;

pub fn write(crypto_key: &str, entropy: f32, is_output_file: bool) {
    write_to_stdout(crypto_key, entropy);
    if is_output_file {
        write_to_file(crypto_key, entropy)
    }
}

fn write_to_stdout(crypto_key: &str, entropy: f32) {
    println!("{}: {}", crypto_key, entropy)
}

fn write_to_file(crypto_key: &str, entropy: f32) {
    let mut file = match OpenOptions::new()
        .write(true)
        .append(true)
        .open(OUTPUT_FILE_PATH)
    {
        Ok(f) => f,
        Err(_) => {
            eprintln!("{} ./{}", ERROR_OUTPUT_FILE_NOT_WRITEABLE, OUTPUT_FILE_PATH);
            std::process::exit(EXIT_OUTPUT_FILE_NOT_WRITEABLE)
        }
    };

    if let Err(e) = writeln!(file, "{}: {}", crypto_key, entropy) {
        eprintln!("{} {}", ERROR_COULDNT_WRITE_OUTPUT_TO_FILE, e);
    }
}

pub fn recreate_output_file(is_output_file: bool) {
    if is_output_file {
        if Path::new(OUTPUT_FILE_PATH).exists() {
            remove_output_file();
        }
        create_output_file();
    }
}

fn remove_output_file() {
    match fs::remove_file(OUTPUT_FILE_PATH) {
        Ok(f) => f,
        Err(_) => {
            eprintln!("{}", ERROR_EXISTING_OUTPUT_FILE_CAN_NOT_BE_REMOVED);
            std::process::exit(EXIT_EXISTING_OUTPUT_FILE_CANNOT_BE_REMOVED);
        }
    };
}

fn create_output_file() {
    match File::create(OUTPUT_FILE_PATH) {
        Ok(f) => f,
        Err(_) => {
            eprintln!(
                "{} ./{}",
                ERROR_OUTPUT_FILE_IS_NOT_WRITEABLE, OUTPUT_FILE_PATH
            );
            std::process::exit(EXIT_OUTPUT_FILE_NOT_WRITEABLE)
        }
    };
}
