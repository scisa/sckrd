pub mod cli;
pub mod input;
pub mod calculation;
pub mod util;

use cli::arguments::Args;

fn main() {
    // Args::get_args();
    // let args = Args::get_args();
    // println!("{:?}", args);

    let bytes = match input::read_bytes::read_bytes_to_vector("/home/chris/Downloads/test_rust/test.bin") {
        Ok(b) => b,
        Err(_) => std::process::exit(4),
    };

    let key_length_bit = 128;
    let entropy_vec = calculation::entropy::search_crypto_keys(&bytes, key_length_bit);

    for entropy in entropy_vec {
        println!("{}", entropy.entropy);
    }
}
