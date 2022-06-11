use crate::util::global_constants::ENTROPY_DELTA;

pub fn calc_entropy_boundary(key_length_bit: usize) -> f32{
    let key_bytes: f32 = key_length_bit as f32 / 8.0 as f32;
    key_bytes.log2() - ENTROPY_DELTA
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


#[test]
fn calc_entropy_boundary_256() {
    assert_eq!(calc_entropy_boundary(256), 5.0 - ENTROPY_DELTA);
}

#[test]
fn calc_entropy_boundary_128() {
    assert_eq!(calc_entropy_boundary(128), 4.0 - ENTROPY_DELTA);
}

#[test]
fn calc_entropy_boundary_512() {
    assert_eq!(calc_entropy_boundary(512), 6.0 - ENTROPY_DELTA);
}

#[test]
fn is_high_entropy_yes_00() {
    assert_eq!(has_high_entropy(5.0, 5.0), true);
}

#[test]
fn is_high_entropy_yes_01() {
    assert_eq!(has_high_entropy(5.0, 4.75), true);
}

#[test]
fn is_high_entropy_no() {
    assert_eq!(has_high_entropy(4.5, 4.75), false);
}

#[test]
fn calc_entropy_for_scope_vector() {
    let scope_vec: Vec<u8> = vec![
        23, 38, 44, 11, 47, 13, 20, 18, 4, 90, 89, 64, 220, 138, 136, 180, 24, 254, 8, 88,
    ];
    assert_eq!(calc_entropy_per_candidate_key(&scope_vec), 4.321928);
}
