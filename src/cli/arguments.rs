use clap::{Arg, ArgMatches, Command};
use std::error::Error;

use crate::cli::arg_constants::*;
use crate::util::error_messages::*;
use crate::util::exit_codes::*;
use crate::util::global_constants::*;

#[derive(Debug)]
pub struct Args {
    pub input_file: String,
    pub output_file: bool,
    pub keysize: usize,
    pub timer: bool,
    pub thread_count: usize,
    pub basic_output: bool,
    pub verbose: bool,
    pub byte_count: usize,
    pub entropy_delta: f32,
}

impl Args {
    pub fn get_args() -> Self {
        let matches = Self::parse_args();
        let args = match matches {
            Ok(m) => m,
            Err(_) => {
                eprintln!("{}", ERROR_EXTRACTING_ARGS_NOT_POSSIBLE);
                std::process::exit(EXIT_EXTRACTING_ARGS_FAILED)
            }
        };

        Self {
            input_file: Self::extract_input_file(&args),
            output_file: Self::extract_output_file(&args),
            keysize: Self::extract_keysize(&args),
            timer: Self::extract_timer(&args),
            thread_count: Self::extract_thread_count(&args),
            basic_output: Self::extract_basic_output(&args),
            verbose: Self::extract_verbose(&args),
            byte_count: Self::extract_byte_count(&args),
            entropy_delta: Self::extract_entropy_delta(&args),
        }
    }

    fn parse_args() -> Result<ArgMatches, Box<dyn Error>> {
        let matches = Command::new(APP_NAME)
            .version(APP_VERSION)
            .author(APP_AUTHOR)
            .about(APP_DESCRIPTION)
            .arg(
                Arg::new(KEY_INPUT_FILE)
                    .help(HELP_INPUT_FILE)
                    .value_name(VALUE_INPUT_FILE)
                    .required(true)
                    .short('i')
                    .takes_value(true)
                    .long(LONG_ARG_INPUT_FILE),
            )
            .arg(
                Arg::new(KEY_OUTPUT_FILE)
                    .help(HELP_OUTPUT_FILE)
                    .value_name(VALUE_OUTPUT_FILE)
                    .required(false)
                    .short('o')
                    .takes_value(false)
                    .long(LONG_ARG_OUTPUT_FILE),
            )
            .arg(
                Arg::new(KEY_KEYSIZE)
                    .help(HELP_KEYSIZE)
                    .value_name(VALUE_KEYSIZE)
                    .required(false)
                    .short('k')
                    .takes_value(true)
                    .long(LONG_ARG_KEYSIZE)
                    .default_value(DEFAULT_VALUE_KEYSIZE),
            )
            .arg(
                Arg::new(KEY_TIMER)
                    .help(HELP_TIMER)
                    .value_name(VALUE_TIMER)
                    .required(false)
                    .short('t')
                    .takes_value(false)
                    .long(LONG_ARG_TIMER),
            )
            .arg(
                Arg::new(KEY_THREAD_COUNT)
                    .help(HELP_THREAD_COUNT)
                    .value_name(VALUE_THREAD_COUNT)
                    .required(false)
                    .short('n')
                    .takes_value(true)
                    .long(LONG_ARG_THREAD_COUNT)
                    .default_value(DEFAULT_VALUE_THREAD_COUNT),
            )
            .arg(
                Arg::new(KEY_BASIC_OUTPUT)
                    .help(HELP_BASIC_OUTPUT)
                    .value_name(VALUE_BASIC_OUTPUT)
                    .required(false)
                    .short('p')
                    .takes_value(false)
                    .long(LONG_ARG_BASIC_OUTPUT),
            )
            .arg(
                Arg::new(KEY_VERBOSE)
                    .help(HELP_VERBOSE)
                    .value_name(VALUE_VERBOSE)
                    .required(false)
                    .short('v')
                    .takes_value(false)
                    .long(LONG_ARG_VERBOSE)
                    .conflicts_with(KEY_BASIC_OUTPUT),
            )
            .arg(
                Arg::new(KEY_BYTE_COUNT)
                    .help(HELP_BYTE_COUNT)
                    .value_name(VALUE_BYTE_COUNT)
                    .required(false)
                    .short('b')
                    .takes_value(true)
                    .long(LONG_ARG_BYTE_COUNT)
                    .default_value(DEFAULT_VALUE_BYTE_COUNT),
            )
            .arg(
                Arg::new(KEY_ENTROPY_DELTA)
                    .help(HELP_ENTROPY_DELTA)
                    .value_name(VALUE_ENTROPY_DELTA)
                    .required(false)
                    .short('e')
                    .takes_value(true)
                    .long(LONG_ARG_ENTROPY_DELTA)
                    .default_value(DEFAULT_VALUE_ENTROPY_DELTA),
            )
            .get_matches();
        Ok(matches)
    }

    fn extract_input_file(args: &ArgMatches) -> String {
        args.value_of(KEY_INPUT_FILE).unwrap().to_string()
    }

    fn extract_output_file(args: &ArgMatches) -> bool {
        args.is_present(KEY_OUTPUT_FILE)
    }

    fn extract_keysize(args: &ArgMatches) -> usize {
        let string_key = args.value_of(KEY_KEYSIZE).unwrap().to_string();
        match string_key.parse::<usize>() {
            Ok(k) => k,
            Err(_) => {
                eprintln!("{}", ERROR_KEYSIZE_NOT_U32);
                std::process::exit(EXIT_KEYSIZE_ARG_IS_UNSIGNED_INT)
            }
        }
    }

    fn extract_timer(args: &ArgMatches) -> bool {
        args.is_present(KEY_TIMER)
    }

    fn extract_thread_count(args: &ArgMatches) -> usize {
        let string_key = args.value_of(KEY_THREAD_COUNT).unwrap().to_string();
        match string_key.parse::<usize>() {
            Ok(k) => k,
            Err(_) => {
                eprintln!("{}", ERROR_THREAD_COUNT_NOT_U32);
                std::process::exit(EXIT_THREAD_COUNT_ARG_IS_UNSIGNED_INT)
            }
        }
    }

    fn extract_basic_output(args: &ArgMatches) -> bool {
        args.is_present(KEY_BASIC_OUTPUT)
    }

    fn extract_verbose(args: &ArgMatches) -> bool {
        args.is_present(KEY_VERBOSE)
    }

    fn extract_byte_count(args: &ArgMatches) -> usize {
        let string_byte_count = args.value_of(KEY_BYTE_COUNT).unwrap().to_string();
        match string_byte_count.parse::<usize>() {
            Ok(k) => k,
            Err(_) => {
                eprintln!("{}", ERROR_BYTE_COUNT_NOT_U32);
                std::process::exit(EXIT_BYTE_COUNT_ARG_IS_UNSIGNED_INT)
            }
        }
    }

    fn extract_entropy_delta(args: &ArgMatches) -> f32 {
        let string_entropy_delta = args.value_of(KEY_ENTROPY_DELTA).unwrap().to_string();
        match string_entropy_delta.parse::<f32>() {
            Ok(e) => e,
            Err(_) => {
                eprintln!("{}", ERROR_ENTROPY_DELTA_NOT_F32);
                std::process::exit(EXIT_ENTROPY_DELTA_ARG_IS_F32)
            }
        }
    }
}
