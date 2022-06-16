use std::sync::{Arc, Mutex};
use hex;
use std::fs::File;

use crate::calculation;
use crate::input;
use crate::output;

use crate::cli::arguments::Args;
use crate::output::write_ck::WriteOptions;
use crate::time::timer::Timer;
use crate::calculation::counter::Counter;

pub fn run() {
    // fetch args
    let args = Args::get_args();

    // initialize timer
    let timer = Timer::new();

    // create result counter
    let result_counter: Counter = Counter::new();

    // fetch bytes
    let mut bytes: Vec<u8> =
        input::read_bytes::get_bytes(args.input_file.as_str(), args.byte_count);

    // create helper variables
    let key_length_byte: usize = calculation::entropy::calc_key_length_byte(args.keysize);
    let bytes_length = bytes.len();

    // calc entropy boundary
    let entropy_delta = calculation::entropy::calc_entropy_delta(args.entropy_delta);
    let entropy_boundary = calculation::entropy::calc_entropy_boundary(args.keysize, entropy_delta);

    // create smartpointer arc for counter
    let result_counter_arc = Arc::new(Mutex::new(result_counter));
    let result_counter_show_arc = Arc::clone(&result_counter_arc);

    // check if analysation has to be done
    if bytes_length >= key_length_byte {
        // calculate number of parallel tasks
        let thread_count = calculation::parallelism::calc_thread_count(
            bytes_length,
            args.thread_count,
            key_length_byte,
        );

        // split vector into pieces for threading
        let split_vec: Vec<Vec<u8>> = calculation::parallelism::split_bytes_vector_for_threading(
            &mut bytes,
            key_length_byte,
            thread_count,
        );

        // recreate output.sckrd if necessary
        output::write_ck::remove_output_file(args.output_file);

        // create write options
        let file = output::write_ck::get_file();
        let write_options: WriteOptions =
            WriteOptions::new(args.output_file, args.basic_output, args.verbose, args.suppress_output);

        
        // create smartpointer variables for threading
        let split_vector_arc = Arc::new(split_vec);
        let write_options_arc = Arc::new(write_options);
        let file_arc = Arc::new(Mutex::new(file));

        // analyse bytes dump
        analyse_bytes_dump(
            split_vector_arc,
            write_options_arc,
            file_arc,
            result_counter_arc,
            thread_count,
            key_length_byte,
            entropy_boundary
        );
    }

    show_counter_if_needed(result_counter_show_arc, args.result_counter);
    show_timer_if_needed(&timer, args.timer)
}

fn analyse_bytes_dump(
    split_vector_arc: Arc<Vec<Vec<u8>>>,
    write_options_arc: Arc<WriteOptions>,
    file_arc: Arc<Mutex<File>>,
    result_counter_arc: Arc<Mutex<Counter>>,
    thread_count: usize,
    key_length_byte: usize,
    entropy_boundary: f32,
) {
    let split_vec_len: usize = split_vector_arc.len();
    if split_vec_len == thread_count && thread_count > 1 {
        let mut thread_handles = vec![];
        for current_thread in 0..thread_count {
            let split_vector_arc = Arc::clone(&split_vector_arc);
            let write_options_arc = Arc::clone(&write_options_arc);
            let file_arc = Arc::clone(&file_arc);
            let result_counter_arc = Arc::clone(&result_counter_arc);
            thread_handles.push(std::thread::spawn(move || {
                run_entropy_analysis(
                    split_vector_arc,
                    write_options_arc,
                    file_arc,
                    result_counter_arc,
                    key_length_byte,
                    current_thread,
                    entropy_boundary
                );
            }));
        }

        for handle in thread_handles {
            handle.join().unwrap();
        }
    } else {
        run_entropy_analysis(split_vector_arc, write_options_arc, file_arc, result_counter_arc, key_length_byte, 0, entropy_boundary);
    }
}

fn run_entropy_analysis(
    bytes_arc: Arc<Vec<Vec<u8>>>,
    write_options_arc: Arc<WriteOptions>,
    file_arc: Arc<Mutex<File>>,
    result_counter_arc: Arc<Mutex<Counter>>,
    key_length_byte: usize,
    current_thread: usize,
    entropy_boundary: f32,
) {
    let bytes_length = bytes_arc[current_thread].len();
    for j in 0..(bytes_length - key_length_byte) {
        let mut scope_vec: Vec<u8> = vec![0; key_length_byte];
        scope_vec.copy_from_slice(&bytes_arc[current_thread][j..(j + key_length_byte)]);
        if calculation::exclution::contains_no_non_hash_characters(&scope_vec) {
            let entropy: f32 = calculation::entropy::calc_entropy_per_candidate_key(&scope_vec);
            if calculation::entropy::has_high_entropy(entropy, entropy_boundary) {
                let crypto_key = hex::encode(scope_vec);
                let file_arc = Arc::clone(&file_arc);
                output::write_ck::write(crypto_key.as_str(), entropy, key_length_byte, &write_options_arc, file_arc);
                let mut counter_lock = result_counter_arc.lock().unwrap();
                counter_lock.increment()
            }
        }
    }
}

fn show_counter_if_needed(result_counter: Arc<Mutex<Counter>>, show_counter: bool) {
    // show counter
    if show_counter {
        let counter_guard = result_counter.lock().unwrap();
        counter_guard.print_result();
    }
}

fn show_timer_if_needed(timer: &Timer, show_timer: bool) {
    // show execution time
    if show_timer {
        timer.print_elapsed_time();
    }
}
