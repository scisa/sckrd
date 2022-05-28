#[derive(Clone)]
pub struct Entropy {
    pub bytes: Vec<u8>,
    pub entropy: f32,
}


pub fn search_crypto_keys(bytes: &Vec<u8>, key_length_bit: usize) -> Vec<Entropy>{
    let entropy_vec_length = bytes.len() - (key_length_bit / 8);
    let mut entropy_vec: Vec<Entropy> = vec![Entropy{bytes: vec![0; key_length_bit / 8] , entropy: 0.0}; entropy_vec_length];
    split_bytes_to_key_scopes(&mut entropy_vec, &bytes, key_length_bit);
    calc_entropy_per_piece(&mut entropy_vec);

    entropy_vec
}

fn split_bytes_to_key_scopes(entropy_vec: &mut Vec<Entropy>, bytes: &Vec<u8>, key_length_bit: usize) {
    let bytes_length = bytes.len();
    let key_length_byte = key_length_bit / 8;
    if bytes_length >= key_length_byte {
        for i in 0..(bytes_length - key_length_byte) {
            let mut scope_vec: Vec<u8> = Vec::new();
            for j in 0..key_length_byte {
                scope_vec.push(bytes[i + j]);
            }
            println!("Scope Vector {} found", i);
            entropy_vec[i].bytes = scope_vec;
        }
    }
}

fn calc_entropy_per_piece(entropy_vec: &mut Vec<Entropy>) {
    let entropy_vec_length = entropy_vec.len();
    for i in 0..entropy_vec_length {
        entropy_vec[i].entropy = entropy::shannon_entropy(&entropy_vec[i].bytes)
    }
}

