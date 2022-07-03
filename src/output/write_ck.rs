use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::sync::{Arc, Mutex};

use crate::util::error_messages::*;
use crate::util::exit_codes::*;
use crate::util::global_constants::{
    DEFAULT_OUTPUT_PATH, ENV_HOME_KEY, OUTPUT_FILE_NAME, OUTPUT_FOLDER,
};

pub struct WriteOptions {
    pub is_output_file: bool,
    pub is_basic_output: bool,
    pub is_verbose: bool,
    pub is_suppress_output: bool,
}

impl WriteOptions {
    pub fn new(
        is_output_file: bool,
        is_basic_output: bool,
        is_verbose: bool,
        is_suppress_output: bool,
    ) -> Self {
        Self {
            is_output_file: is_output_file,
            is_basic_output: is_basic_output,
            is_verbose: is_verbose,
            is_suppress_output: is_suppress_output,
        }
    }
}

pub fn write(
    crypto_key: &str,
    entropy: f32,
    key_length_byte: usize,
    write_options: &WriteOptions,
    file: Arc<Mutex<Option<File>>>,
) {
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

pub fn get_file(is_output_file: bool) -> Option<File> {
    let mut output_file_option: Option<File> = None;

    if is_output_file {
        let output_file = get_output_file();

        let file = match OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(output_file)
        {
            Ok(file) => file,
            Err(e) => {
                eprintln!("{}: {}", ERROR_OUTPUT_FILE_CAN_NOT_BE_CREATED, e);
                std::process::exit(EXIT_OUTPUT_FILE_CAN_NOT_BE_CREATED);
            }
        };

        output_file_option = Some(file);
    }

    output_file_option
}

fn write_to_file(crypto_key: &str, entropy: f32, file_option: Arc<Mutex<Option<File>>>) {
    let mut file_guard = file_option.lock().unwrap();
    let data = format!("{}: {}\n", crypto_key, entropy);

    match &mut *file_guard {
        Some(file) => {
            if let Err(e) = file.write_all(data.as_bytes()) {
                eprintln!("{}: {}", ERROR_OUTPUT_FILE_IS_NOT_WRITEABLE, e);
            }
        }
        None => {}
    }
}

pub fn remove_output_file(is_output_file: bool) {
    let output_file = get_output_file();

    if is_output_file {
        if Path::new(&output_file).exists() {
            match fs::remove_file(output_file) {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("{}: {}", ERROR_EXISTING_OUTPUT_FILE_CAN_NOT_BE_REMOVED, e);
                    std::process::exit(EXIT_EXISTING_OUTPUT_FILE_CANNOT_BE_REMOVED);
                }
            };
        }
    }
}

pub fn create_output_folder_if_not_exists() {
    let output_file_path = get_output_file_path();
    if let Err(e) = fs::create_dir_all(output_file_path) {
        eprintln!("{}: {}", ERROR_OUTPUT_FOLDER_CAN_NOT_BE_CREATED, e);
        std::process::exit(EXIT_OUTPUT_FOLDER_CAN_NOT_BE_CREATED);
    };
}

fn get_output_file_path() -> String {
    let home = match env::var(ENV_HOME_KEY) {
        Ok(p) => p,
        Err(_) => String::from(DEFAULT_OUTPUT_PATH),
    };

    home + "/" + OUTPUT_FOLDER
}

fn get_output_file() -> String {
    get_output_file_path() + "/" + OUTPUT_FILE_NAME
}
