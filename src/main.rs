pub mod calculation;
pub mod cli;
pub mod input;
pub mod output;
pub mod time;
pub mod util;

use std::sync::Arc;

use cli::arguments::Args;
use output::write_ck::WriteOptions;

fn main() {
    let args = Args::get_args();

    let timer = time::timer::Timer::new();
    let mut bytes: Vec<u8> = input::read_bytes::get_bytes(args.input_file.as_str(), args.byte_count);
    println!("{}", bytes.len());

    let key_length_byte: usize = args.keysize / 8;
    let bytes_length = bytes.len();

    if bytes_length > key_length_byte {
        let split_vec: Vec<Vec<u8>> =
        calculation::parallelism::split_bytes_vector(&mut bytes, key_length_byte, args.thread_count);
        let split_vec_len: usize = split_vec.len();

        // create write options
        let write_options: WriteOptions = WriteOptions::new(args.output_file, args.basic_output, args.verbose);

        // create variables for threading
        let bytes_arc = Arc::new(split_vec);
        let write_options_arc = Arc::new(write_options);

        // recreate output.sckrd if necessary
        output::write_ck::recreate_output_file(args.output_file);

        // run entropy analysis
        if bytes_length >= key_length_byte {  
            if split_vec_len == args.thread_count && args.thread_count > 1 {
                let mut thread_handles = vec![];
                println!("there");
                for current_thread in 0..args.thread_count {
                    let bytes_arc = Arc::clone(&bytes_arc);
                    let write_options_arc = Arc::clone(&write_options_arc);
                    thread_handles.push(std::thread::spawn(move || {
                        run_entropy_analysis(bytes_arc, key_length_byte, current_thread, &write_options_arc);
                    }));
                }

                for handle in thread_handles {
                    handle.join().unwrap();
                }
            } else {
                run_entropy_analysis(bytes_arc, key_length_byte, 0, &write_options_arc);
            }
            
        }
    }
    
    // show execution time
    if args.timer {
        timer.print_elapsed_time();
    }
}

fn run_entropy_analysis(bytes_arc: Arc<Vec<Vec<u8>>>, key_length_byte: usize, current_thread: usize, write_options: &WriteOptions) {
    let bytes_length = bytes_arc[current_thread].len();
    for j in 0..(bytes_length - key_length_byte) {
        let mut scope_vec: Vec<u8> = vec![0; key_length_byte];
        scope_vec.copy_from_slice(&bytes_arc[current_thread][j..(j + key_length_byte)]);
        if calculation::exclution::contains_non_hash_characters(&scope_vec) {
            let entropy: f32 = calculation::entropy::calc_entropy_per_key_attempt(&scope_vec);
            if calculation::entropy::is_high_entropy(entropy) {
                if let Ok(crypto_key) = std::str::from_utf8(&scope_vec) {
                    output::write_ck::write(crypto_key, entropy, key_length_byte ,write_options);
                }
            }
        }
    }
}
