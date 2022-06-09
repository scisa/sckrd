use crate::util::global_constants::ENTROPY_BOUNDARY;

pub fn calc_entropy_per_candidate_key(scope_vec: &Vec<u8>) -> f32 {
    entropy::shannon_entropy(&scope_vec)
}

pub fn has_high_entropy(entropy: f32) -> bool {
    if entropy >= ENTROPY_BOUNDARY {
        return true;
    }
    return false;
}

#[test]
fn is_high_entropy_yes_00() {
    assert_eq!(has_high_entropy(ENTROPY_BOUNDARY), true);
}

#[test]
fn is_high_entropy_yes_01() {
    assert_eq!(has_high_entropy(ENTROPY_BOUNDARY + 1.0), true);
}

#[test]
fn is_high_entropy_no() {
    assert_eq!(has_high_entropy(ENTROPY_BOUNDARY - 1.0), false);
}

#[test]
fn calc_entropy_for_scope_vector() {
    let scope_vec: Vec<u8> = vec![23, 38, 44, 11, 47, 13, 20, 18, 4, 90, 89, 64, 220, 138, 136, 180, 24, 254, 8, 88];
    assert_eq!(calc_entropy_per_candidate_key(&scope_vec), 4.321928);
}