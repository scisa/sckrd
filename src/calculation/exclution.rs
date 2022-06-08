use crate::util::global_constants::EXCLUDES;

pub fn contains_non_hash_characters(scope_vec: &Vec<u8>) -> bool {
    for exclude in EXCLUDES {
        if scope_vec.iter().any(|v| v == &exclude) {
            return false;
        }
    }
    return true;
}

#[test]
fn contains_non_hashable_character_newline() {
    let scope_vec: Vec<u8> = vec![15, 30, 47, 11, 10, 90, 111, 120, 35, 210];

    assert_eq!(contains_non_hash_characters(&scope_vec), false);
}

#[test]
fn contains_non_hashable_character_space() {
    let scope_vec: Vec<u8> = vec![15, 30, 47, 11, 32, 90, 111, 120, 35, 210];

    assert_eq!(contains_non_hash_characters(&scope_vec), false);
}

#[test]
fn contains_non_hashable_character_exclamation_mark() {
    let scope_vec: Vec<u8> = vec![15, 30, 47, 11, 33, 90, 111, 120, 35, 210];

    assert_eq!(contains_non_hash_characters(&scope_vec), false);
}

#[test]
fn contains_non_hashable_character_dollar() {
    let scope_vec: Vec<u8> = vec![15, 30, 47, 11, 36, 90, 111, 120, 35, 210];

    assert_eq!(contains_non_hash_characters(&scope_vec), false);
}

#[test]
fn contains_non_hashable_character_asterisk() {
    let scope_vec: Vec<u8> = vec![15, 30, 47, 11, 42, 90, 111, 120, 35, 210];

    assert_eq!(contains_non_hash_characters(&scope_vec), false);
}

#[test]
fn contains_non_hashable_character_colon() {
    let scope_vec: Vec<u8> = vec![15, 30, 47, 11, 58, 90, 111, 120, 35, 210];

    assert_eq!(contains_non_hash_characters(&scope_vec), false);
}

#[test]
fn contains_non_hashable_character_semicolon() {
    let scope_vec: Vec<u8> = vec![15, 30, 47, 11, 59, 90, 111, 120, 35, 210];

    assert_eq!(contains_non_hash_characters(&scope_vec), false);
}

#[test]
fn contains_non_hashable_character_backslash() {
    let scope_vec: Vec<u8> = vec![15, 30, 47, 11, 92, 90, 111, 120, 35, 210];

    assert_eq!(contains_non_hash_characters(&scope_vec), false);
}

#[test]
fn contains_non_hashable_characters() {
    let scope_vec: Vec<u8> = vec![10, 30, 47, 11, 92, 90, 111, 59, 35, 210];

    assert_eq!(contains_non_hash_characters(&scope_vec), false);
}

#[test]
fn contains_no_non_hashable_characters() {
    let scope_vec: Vec<u8> = vec![15, 30, 47, 11, 9, 90, 111, 120, 35, 210];

    assert_eq!(contains_non_hash_characters(&scope_vec), true);
}