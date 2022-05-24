use clap::{Arg, ArgMatches, Command};
use std::error::Error;

use crate::util::global_constants::*;
use crate::util::exit_codes::*;
use crate::cli::arg_constants::*;


#[derive(Debug)]
pub struct Args {
    input_file: String,
    output_file: bool,
}

impl Args {
    pub fn get_args() -> Self {
        let matches = Self::parse_args();
        let args = match matches {
            Ok(m) => m,
            Err(_) => std::process::exit(EXIT_EXTRACTING_ARGS_FAILED),
        };

        Self {
            input_file: Self::extract_input_file(&args),
            output_file: Self::extract_output_file(&args),
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
                .required(false)
                .short('i')
                .takes_value(true)
                .long(LONG_ARG_INPUT_FILE)
                .default_value("-")
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
            .get_matches();
        Ok(matches)
    }

    fn extract_input_file(args: &ArgMatches) -> String {
        args.value_of(KEY_INPUT_FILE).unwrap().to_string()
    }

    fn extract_output_file(args: &ArgMatches) -> bool {
        args.is_present(KEY_OUTPUT_FILE)
    }
}
