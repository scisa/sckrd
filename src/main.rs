pub mod calculation;
pub mod cli;
pub mod input;
pub mod output;
pub mod time;
pub mod util;

use std::sync::Arc;

use cli::arguments::Args;

fn main() {
    let args = Args::get_args();

    let timer = time::timer::Timer::new();
    let bytes: Vec<u8> = input::read_bytes::get_bytes(args.input_file.as_str());

    let key_length_byte: usize = args.keysize / 8;
    let bytes_length = bytes.len();
    let split_vec: Vec<Vec<u8>> =
        calculation::parallelism::split_bytes_vector(&bytes, key_length_byte, args.thread_count);

    // create variables for threading
    let bytes_arc = Arc::new(split_vec);
    let args_arc = Arc::new(args);

    // recreate output.sckrd if necessary
    output::write_ck::recreate_output_file(args_arc.output_file);

    if bytes_length >= key_length_byte {   
        let mut thread_handles = vec![];
        for current_thread in 0..args_arc.thread_count {
            let bytes_arc = Arc::clone(&bytes_arc);
            let args_arc = Arc::clone(&args_arc);
            thread_handles.push(std::thread::spawn(move || {
                thread_function(bytes_arc, key_length_byte, current_thread, &args_arc);
            }));
        }

        for handle in thread_handles {
            handle.join().unwrap();
        }
    }

    if args_arc.timer {
        timer.print_elapsed_time();
    }
}

fn thread_function(bytes_arc: Arc<Vec<Vec<u8>>>, key_length_byte: usize, current_thread: usize, args: &Args) {
    let bytes_length = bytes_arc[current_thread].len();
    for j in 0..(bytes_length - key_length_byte) {
        let mut scope_vec: Vec<u8> = vec![0; key_length_byte];
        scope_vec.copy_from_slice(&bytes_arc[current_thread][j..(j + key_length_byte)]);
        if calculation::exclution::contains_no_hash_characters(&scope_vec) {
            let entropy: f32 = calculation::entropy::calc_entropy_per_key_attempt(&scope_vec);
            if calculation::entropy::is_high_entropy(entropy) {
                if let Ok(crypto_key) = std::str::from_utf8(&scope_vec) {
                    output::write_ck::write(crypto_key, entropy, args.output_file);
                }
            }
        }
    }
}
