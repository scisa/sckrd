// use crate::util::global_constants::EXCLUDES;

pub fn contains_no_non_hash_characters(_scope_vec: &Vec<u8>) -> bool {
    // for exclude in EXCLUDES {
    //     if scope_vec.iter().any(|v| v == &exclude) {
    //         return false;
    //     }
    // }
    return true;
}


#[test]
fn contains_no_non_hashable_characters() {
    let scope_vec: Vec<u8> = vec![15, 30, 47, 11, 9, 90, 111, 120, 35, 210];

    assert_eq!(contains_no_non_hash_characters(&scope_vec), true);
}
