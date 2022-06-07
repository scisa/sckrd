pub mod administration;
pub mod calculation;
pub mod cli;
pub mod input;
pub mod output;
pub mod time;
pub mod util;

fn main() {
    administration::program::run();
}
