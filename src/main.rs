pub mod cli;
pub mod input;
pub mod util;

use cli::arguments::Args;

fn main() {
    // Args::get_args();
    // let args = Args::get_args();
    // println!("{:?}", args);
    let bytes = match input::read_bytes::read_bytes_to_vector("/home/chris/Public/output.lime") {
        Ok(b) => b,
        Err(_) => std::process::exit(4),
    };
}
