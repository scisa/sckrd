use std::io::{self, Read, BufRead, BufReader};
use std::fs::File;

pub fn read_bytes_to_vector(path: &str) -> io::Result<Vec<u8>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buffer: Vec<u8> = Vec::new();
    
    // Read file into vector.
    reader.read_to_end(&mut buffer)?;
    
    Ok(buffer)
}

// pub fn read_bytes_to_vector(path: &str) -> io::Result<()> {
//     const CAP: usize = 1024 * 256;
//     let file = File::open(path)?;
//     let mut reader = BufReader::with_capacity(CAP, file);

//     loop {
//         let length = {
//             let buffer = reader.fill_buf()?;
//             // do stuff with buffer here
//             for value in buffer {
//                 println!("BYTE: {}", value);
//             }
//             buffer.len()
//         };
//         if length == 0 {
//             break;
//         }
//         reader.consume(length);
//     }

//     Ok()
// }