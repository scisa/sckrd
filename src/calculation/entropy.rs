use crate::util::global_constants::*;

pub fn calc_entropy_per_key_attempt(scope_vec: &Vec<u8>) -> f32 {
    entropy::shannon_entropy(&scope_vec)
}

pub fn is_high_entropy(entropy: f32) -> bool{
    if entropy >= ENTROPY_BORDER {
        return true;
    }
    return false;
}