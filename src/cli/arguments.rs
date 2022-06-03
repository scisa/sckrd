use clap::{Arg, ArgMatches, Command};
use std::error::Error;

use crate::util::global_constants::*;
use crate::util::exit_codes::*;
use crate::util::error_messages::*;
use crate::cli::arg_constants::*;


#[derive(Debug)]
pub struct Args {
    pub input_file: String,
    pub output_file: bool,
    pub keysize: usize,
    pub timer: bool,
}

impl Args {
    pub fn get_args() -> Self {
        let matches = Self::parse_args();
        let args = match matches {
            Ok(m) => m,
            Err(_) => {
                eprintln!("{}", ERROR_EXTRACTING_ARGS_NOT_POSSIBLE);
                std::process::exit(EXIT_EXTRACTING_ARGS_FAILED)
            },
        };

        Self {
            input_file: Self::extract_input_file(&args),
            output_file: Self::extract_output_file(&args),
            keysize: Self::extract_keysize(&args),
            timer: Self::extract_timer(&args),
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
                .long(LONG_ARG_INPUT_FILE)
            )
            .arg(
                Arg::new(KEY_OUTPUT_FILE)
                .help(HELP_OUTPUT_FILE)
                .value_name(VALUE_OUTPUT_FILE)
                .required(false)
                .short('o')
                .takes_value(false)
                .long(LONG_ARG_OUTPUT_FILE)
            )
            .arg(
                Arg::new(KEY_KEYSIZE)
                .help(HELP_KEYSIZE)
                .value_name(VALUE_KEYSIZE)
                .required(false)
                .short('k')
                .takes_value(true)
                .long(LONG_ARG_KEYSIZE)
                .default_value(DEFAULT_VALUE_KEYSIZE)
            )
            .arg(
                Arg::new(KEY_TIMER)
                .help(HELP_TIMER)
                .value_name(VALUE_TIMER)
                .required(false)
                .short('t')
                .takes_value(false)
                .long(LONG_ARG_TIMER)
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
                std::process::exit(EXIT_KEY_SIZE_ARG_IS_UNSIGNED_INT)
            }
        }
    }

    fn extract_timer(args: &ArgMatches) -> bool {
        args.is_present(KEY_TIMER)
    }
}
