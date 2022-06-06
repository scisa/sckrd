pub fn split_bytes_vector(bytes: &Vec<u8>, key_length_byte: usize, n: usize) -> Vec<Vec<u8>> {
    let bytes_len: usize = bytes.len();
    let mut split_vec: Vec<Vec<u8>> = Vec::new();

    for chunk in bytes.chunks(bytes_len / n) {
        split_vec.push(chunk.to_vec());
    }
    println!("{}", split_vec.len());

    // scope_vec.copy_from_slice(bytes[i..(i + key_length_byte)]);
    split_vec
}
