use std::collections::HashSet;

const ASCII_ALPHA_LEN: usize = 26;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let mut set = HashSet::new();

    for b in sentence.as_bytes() {
        if !b.is_ascii_alphabetic() {
            continue;
        }

        set.insert(b.to_ascii_lowercase());
    }

    set.len() == ASCII_ALPHA_LEN
}
