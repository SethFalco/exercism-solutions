use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let normalized: String = words.as_bytes().iter().map(|&b| {
        if b.is_ascii_alphanumeric() || b == b'\'' {
            return char::from(b);
        }

        char::from(b' ')
    }).collect();

    normalized.split_ascii_whitespace().fold(HashMap::new(), |mut acc, word| {
        let mut effective_word = word.to_owned();

        if word.starts_with("'") && word.ends_with("'") {
            let slice = effective_word.as_bytes()[1..effective_word.len() - 1].to_vec();
            effective_word = String::from_utf8(slice).unwrap();
        }

        *acc.entry(effective_word.to_ascii_lowercase()).or_insert(0u32) += 1;

        acc
    })
}
