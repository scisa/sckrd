pub fn split_bytes_vector_bak(bytes: &Vec<u8>, _key_length_byte: usize, n: usize) -> Vec<Vec<u8>> {
    let bytes_len: usize = bytes.len();
    let mut split_vec: Vec<Vec<u8>> = Vec::new();

    for chunk in bytes.chunks(bytes_len / n) {
        split_vec.push(chunk.to_vec());
    }
    println!("{}", split_vec.len());

    // scope_vec.copy_from_slice(bytes[i..(i + key_length_byte)]);
    split_vec
}

pub fn split_bytes_vector(bytes: &mut Vec<u8>, key_length_byte: usize, n: usize) -> Vec<Vec<u8>> {
    let mut bytes_len: usize = bytes.len();
    let mut split_vec: Vec<Vec<u8>> = Vec::new();
    let mut overlap_vector: Vec<u8>;

    if bytes_len > key_length_byte * 8 {
        for i in 0..n {
            if bytes_len >= n {
                let mut temp_vec: Vec<u8> = bytes.split_off(bytes_len - (bytes_len / 8));
                if !split_vec.is_empty() {
                    overlap_vector = create_overlap(&split_vec, key_length_byte, i);
                    temp_vec.append(&mut overlap_vector);
                }
    
                bytes_len = bytes.len();
                split_vec.push(temp_vec);
            } else {
                overlap_vector = create_overlap(&split_vec, key_length_byte, i);
                bytes.append(&mut overlap_vector);
                split_vec.push(bytes.to_vec());
                break;
            }
        }
    } else {
        split_vec.push(bytes.to_vec());
    }
    

    split_vec
}

// fn split_bytes(bytes: &mut Vec<u8>, key_length_byte: usize, n: usize) {
//     for i in 0..n {
//         if bytes_len >= n {
//             let mut temp_vec: Vec<u8> = bytes.split_off(bytes_len - (bytes_len/8));
//             if !split_vec.is_empty() {
//                 overlap_vector = create_overlap(&split_vec, key_length_byte, i);
//                 temp_vec.append(&mut overlap_vector);
//             }

//             bytes_len = bytes.len();
//             split_vec.push(temp_vec);
//         } else {
//             overlap_vector = create_overlap(&split_vec, key_length_byte, i);
//             bytes.append(&mut overlap_vector);
//             split_vec.push(bytes.to_vec());
//             break;
//         }
//     }
// }

fn create_overlap(split_vec: &Vec<Vec<u8>>, key_length_byte: usize, current_thread: usize,) -> Vec<u8> {
    let mut overlap_vector: Vec<u8> = vec![0; key_length_byte];
    overlap_vector.copy_from_slice(&split_vec[current_thread - 1][0..key_length_byte]);

    // let len_overlap: usize = split_vec[current_thread - 1].len();
    // let overlap_vector: Vec<u8> = Vec::new();
    // if len_overlap >= key_length_byte {
    //     let mut overlap_vector: Vec<u8> = vec![0; key_length_byte];
    //     overlap_vector.copy_from_slice(&split_vec[current_thread - 1][0..key_length_byte]);
    // } else {
    //     let mut overlap_vector: Vec<u8> = vec![0; len_overlap];
    //     overlap_vector.copy_from_slice(&split_vec[current_thread - 1][0..len_overlap]);
    // }

    overlap_vector
}
