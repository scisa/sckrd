pub fn split_bytes_vector_for_threading(
    bytes: &mut Vec<u8>,
    key_length_byte: usize,
    n: usize,
) -> Vec<Vec<u8>> {
    
    let mut split_vec: Vec<Vec<u8>> = Vec::new();

    if bytes.len() > key_length_byte * n && n > 1 {
        split_up_bytes(
            bytes,
            &mut split_vec,
            key_length_byte,
            n,
        );
    } else {
        split_vec.push(bytes.to_vec());
    }

    split_vec
}

fn split_up_bytes(
    bytes: &mut Vec<u8>,
    split_vec: &mut Vec<Vec<u8>>,
    key_length_byte: usize,
    n: usize,
) {
    let bytes_len_init: usize = bytes.len();
    let mut overlap_vector: Vec<u8> = Vec::new();
    for i in 0..n {
        if i < n - 1 {
            create_full_split_vector(
                bytes,
                split_vec,
                &mut overlap_vector,
                key_length_byte,
                bytes_len_init,
                i,
                n,
            );
        } else {
            create_rest_split_vector(bytes, split_vec, &mut overlap_vector, key_length_byte, i);
            break;
        }
    }
}

fn create_full_split_vector(
    bytes: &mut Vec<u8>,
    split_vec: &mut Vec<Vec<u8>>,
    overlap_vector: &mut Vec<u8>,
    key_length_byte: usize,
    bytes_len_init: usize,
    i: usize,
    n: usize,
) {
    let bytes_len = bytes.len();
    let mut temp_vec: Vec<u8> = bytes.split_off(bytes_len - (bytes_len_init / n));
    if !split_vec.is_empty() {
        *overlap_vector = create_overlap(&split_vec, key_length_byte, i);
        temp_vec.append(overlap_vector);
    }
    split_vec.push(temp_vec);
}

fn create_rest_split_vector(
    bytes: &mut Vec<u8>,
    split_vec: &mut Vec<Vec<u8>>,
    overlap_vector: &mut Vec<u8>,
    key_length_byte: usize,
    i: usize,
) {
    *overlap_vector = create_overlap(&split_vec, key_length_byte, i);
    bytes.append(overlap_vector);
    split_vec.push(bytes.to_vec());
}

fn create_overlap(
    split_vec: &Vec<Vec<u8>>,
    key_length_byte: usize,
    current_thread: usize,
) -> Vec<u8> {
    let mut overlap_vector: Vec<u8> = vec![0; key_length_byte - 1];
    overlap_vector.copy_from_slice(&split_vec[current_thread - 1][0..key_length_byte - 1]);

    overlap_vector
}

pub fn calc_thread_count(
    bytes_length: usize,
    mut thread_count: usize,
    key_length_byte: usize,
) -> usize {
    if thread_count == 0 {
        thread_count = 1;
    }
    while (bytes_length / thread_count) < key_length_byte * 2 {
        thread_count /= 2;
    }

    thread_count
}

#[test]
fn calc_thread_count_0() {
    // bytes_length >= key_length_byte
    // key_length_byte >= 8
    assert_eq!(calc_thread_count(500, 0, 32), 1);
    assert_eq!(calc_thread_count(32, 0, 32), 1);
    assert_eq!(calc_thread_count(150, 0, 32), 1);
}

#[test]
fn calc_thread_count_very_high_but_equal_bytes_length() {
    assert_eq!(calc_thread_count(500, 500, 32), 15);
}

#[test]
fn calc_thread_count_very_high_but_lower_bytes_length() {
    assert_eq!(calc_thread_count(500, 600, 32), 9);
}

#[test]
fn calc_thread_count_very_high_but_higher_bytes_length() {
    assert_eq!(calc_thread_count(500, 400, 32), 12);
}

#[test]
fn calc_thread_count_1() {
    assert_eq!(calc_thread_count(500, 1, 32), 1);
}

#[test]
fn calc_thread_count_2() {
    assert_eq!(calc_thread_count(500, 2, 32), 2);
}

#[test]
fn calc_thread_count_4() {
    assert_eq!(calc_thread_count(500, 4, 32), 4);
}

#[test]
fn calc_thread_count_8() {
    assert_eq!(calc_thread_count(500, 8, 32), 8);
}

#[test]
fn calc_thread_count_15() {
    assert_eq!(calc_thread_count(500, 15, 32), 15);
}

#[test]
fn calc_thread_count_16() {
    assert_eq!(calc_thread_count(500, 16, 32), 8);
}

#[test]
fn calc_thread_count_32() {
    assert_eq!(calc_thread_count(500, 32, 32), 8);
}
