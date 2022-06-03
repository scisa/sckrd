pub mod cli;
pub mod input;
pub mod calculation;
pub mod output;
pub mod time;
pub mod util;

use cli::arguments::Args;

fn main() {
    let args = Args::get_args();
    println!("{:?}", args);

    // let file = "/home/chris/Downloads/test_rust/test_small.bin";
    // extract args variables
    let file = args.input_file.as_str();
    let key_length_bit = args.keysize;
    let use_timer = args.timer;

    let timer = time::timer::Timer::new();


    let bytes = input::read_bytes::get_bytes(file);

    let bytes_length = bytes.len();
    let key_length_byte = key_length_bit / 8;

    output::write_ck::recreate_output_file(args.output_file);

    if bytes_length >= key_length_byte {
        for i in 0..(bytes_length - key_length_byte) {
            let mut scope_vec: Vec<u8> = vec![0; key_length_byte];
            scope_vec.copy_from_slice(&bytes[i..(i + key_length_byte)]);
            let entropy: f32 = calculation::entropy::calc_entropy_per_key_attempt(&scope_vec);
            if calculation::entropy::is_high_entropy(entropy) {
                let crypto_key = match std::str::from_utf8(&scope_vec) {
                    Ok(s) => s,
                    Err(_) => continue,
                };
                output::write_ck::write(crypto_key, entropy, args.output_file);
            }
        }
    }

    if use_timer {
        timer.print_elapsed_time();
    }
}
