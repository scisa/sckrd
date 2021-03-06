use crate::util::global_constants::{
    DEFAULT_ENTROPY_DELTA, GREATEST_KEY_LENGTH_BIT, SMALLEST_KEY_LENGTH_BIT,
};

pub fn calc_entropy_boundary(key_length_bit: usize, entropy_delta: f32) -> f32 {
    let key_bytes: f32 = key_length_bit as f32 / 8.0 as f32;
    let boundary = key_bytes.log2() - entropy_delta;

    if boundary < 0.0 {
        return 0.0;
    }
    return boundary;
}

pub fn calc_entropy_per_candidate_key(scope_vec: &Vec<u8>) -> f32 {
    entropy::shannon_entropy(&scope_vec)
}

pub fn has_high_entropy(entropy: f32, entropy_boundary: f32) -> bool {
    if entropy >= entropy_boundary {
        return true;
    }
    return false;
}

pub fn calc_entropy_delta(user_given_entropy_delta: f32) -> f32 {
    if user_given_entropy_delta < 0.0 {
        return DEFAULT_ENTROPY_DELTA;
    }
    return user_given_entropy_delta;
}

pub fn calc_key_length_byte(user_given_key_length_bit: usize) -> usize {
    let key_length_byte;
    if user_given_key_length_bit < SMALLEST_KEY_LENGTH_BIT {
        key_length_byte = SMALLEST_KEY_LENGTH_BIT / 8;
    } else if user_given_key_length_bit > GREATEST_KEY_LENGTH_BIT {
        key_length_byte = GREATEST_KEY_LENGTH_BIT / 8;
    } else {
        key_length_byte = user_given_key_length_bit / 8;
    }

    key_length_byte
}

#[test]
fn calc_entropy_boundary_256() {
    assert_eq!(calc_entropy_boundary(256, 0.2), 5.0 - 0.2);
}

#[test]
fn calc_entropy_boundary_128() {
    assert_eq!(calc_entropy_boundary(128, 0.2), 4.0 - 0.2);
}

#[test]
fn calc_entropy_boundary_512() {
    assert_eq!(calc_entropy_boundary(512, 0.2), 6.0 - 0.2);
}

#[test]
fn has_high_entropy_yes_00() {
    assert_eq!(has_high_entropy(5.0, 5.0), true);
}

#[test]
fn has_high_entropy_yes_01() {
    assert_eq!(has_high_entropy(5.0, 4.75), true);
}

#[test]
fn has_high_entropy_no() {
    assert_eq!(has_high_entropy(4.5, 4.75), false);
}

#[test]
fn calc_entropy_for_scope_vector() {
    let scope_vec: Vec<u8> = vec![
        23, 38, 44, 11, 47, 13, 20, 18, 4, 90, 89, 64, 220, 138, 136, 180, 24, 254, 8, 88,
    ];
    assert_eq!(calc_entropy_per_candidate_key(&scope_vec), 4.321928);
}

#[test]
fn calc_key_length_byte_byte_0() {
    assert_eq!(calc_key_length_byte(0), SMALLEST_KEY_LENGTH_BIT / 8);
}

#[test]
fn calc_key_length_byte_byte_55() {
    assert_eq!(calc_key_length_byte(55), SMALLEST_KEY_LENGTH_BIT / 8);
}

#[test]
fn calc_key_length_byte_byte_56() {
    assert_eq!(calc_key_length_byte(56), 56 / 8);
}

#[test]
fn calc_key_length_byte_byte_1024() {
    assert_eq!(calc_key_length_byte(1024), 1024 / 8);
}

#[test]
fn calc_key_length_byte_byte_1025() {
    assert_eq!(calc_key_length_byte(1025), GREATEST_KEY_LENGTH_BIT / 8);
}

#[test]
fn calc_key_length_byte_byte_128() {
    assert_eq!(calc_key_length_byte(128), 128 / 8);
}

#[test]
fn calc_key_length_byte_byte_256() {
    assert_eq!(calc_key_length_byte(256), 256 / 8);
}

#[test]
fn calc_key_length_byte_byte_512() {
    assert_eq!(calc_key_length_byte(512), 512 / 8);
}
