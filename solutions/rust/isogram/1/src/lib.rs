use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let alpha: Vec<u8> = candidate.as_bytes().iter()
        .filter(|x| x.is_ascii_alphabetic())
        .map(|x| x.to_ascii_lowercase())
        .collect();
    let set: HashSet<&u8> = alpha.iter().collect();

    alpha.len() == set.len()
}
