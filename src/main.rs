pub mod cli;
pub mod util;

use cli::arguments::Args;

fn main() {
    Args::get_args();
    let args = Args::get_args();
    println!("{:?}", args);
}
