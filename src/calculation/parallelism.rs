pub fn split_bytes_vector(bytes: &mut Vec<u8>, key_length_byte: usize, n: usize) -> Vec<Vec<u8>> {
    let bytes_len: usize = bytes.len();
    let bytes_len_init: usize = bytes.len();
    let mut split_vec: Vec<Vec<u8>> = Vec::new();

    if bytes_len > key_length_byte * 8 && n > 1{
        split_bytes(bytes, &mut split_vec, key_length_byte, n, bytes_len, bytes_len_init);
    } else {
        split_vec.push(bytes.to_vec());
    }
    
    split_vec
}

fn split_bytes(bytes: &mut Vec<u8>, split_vec: &mut Vec<Vec<u8>>, key_length_byte: usize, n: usize, bytes_len: usize, bytes_len_init: usize) {
    let mut overlap_vector: Vec<u8> = Vec::new();
    for i in 0..n {
        if bytes_len >= n {
            create_full_split_vector(bytes, split_vec, &mut overlap_vector, key_length_byte, bytes_len_init, i, n);
        } else {
            create_rest_split_vector(bytes, split_vec, &mut overlap_vector, key_length_byte, i);
            break;
        }
    }
}

fn create_full_split_vector(bytes: &mut Vec<u8>, split_vec: &mut Vec<Vec<u8>>, overlap_vector: &mut Vec<u8>, key_length_byte: usize, bytes_len_init: usize, i: usize, n: usize) {
    let bytes_len = bytes.len();
    let mut temp_vec: Vec<u8> = bytes.split_off(bytes_len - (bytes_len_init / n));
    if !split_vec.is_empty() {
        *overlap_vector = create_overlap(&split_vec, key_length_byte, i);
        temp_vec.append(overlap_vector);
    }
    split_vec.push(temp_vec);
}

fn create_rest_split_vector(bytes: &mut Vec<u8>, split_vec: &mut Vec<Vec<u8>>, overlap_vector: &mut Vec<u8>, key_length_byte: usize, i: usize) {
    *overlap_vector = create_overlap(&split_vec, key_length_byte, i);
    bytes.append(overlap_vector);
    split_vec.push(bytes.to_vec());
}

fn create_overlap(split_vec: &Vec<Vec<u8>>, key_length_byte: usize, current_thread: usize,) -> Vec<u8> {
    let mut overlap_vector: Vec<u8> = vec![0; key_length_byte];
    overlap_vector.copy_from_slice(&split_vec[current_thread - 1][0..key_length_byte]);

    overlap_vector
}

pub fn calc_thread_count(bytes_length: usize, mut thread_count: usize, key_length_byte: usize) -> usize{
    while (bytes_length / thread_count) < key_length_byte {
        thread_count /= 2;
    }
    thread_count
}