use crate::util::global_constants::BYTE_NEWLINE;

pub fn contains_no_hash_characters(scope_vec: &Vec<u8>) -> bool {
    if !scope_vec.contains(&BYTE_NEWLINE) {
        return true;
    }
    return false;
}
