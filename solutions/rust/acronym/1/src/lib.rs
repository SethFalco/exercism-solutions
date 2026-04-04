pub fn abbreviate(phrase: &str) -> String {
    if phrase.is_empty() {
        return String::new();
    }

    let mut indexes = Vec::new();

    for i in 0..phrase.len() - 1 {
        let a = char::from(phrase.bytes().nth(i).unwrap());
        let b = char::from(phrase.bytes().nth(i + 1).unwrap());

        if i == 0 {
            indexes.push(0);
            continue;
        }

        if a.is_uppercase() && b.is_lowercase() {
            indexes.push(i);
            continue;
        }

        if (a.is_ascii_punctuation() || a.is_whitespace()) && a != '\'' && b.is_alphabetic() {
            indexes.push(i + 1);
            continue;
        }
    }

    indexes.dedup();
    indexes.iter().map(|&x| char::from(phrase.bytes().nth(x).unwrap().to_ascii_uppercase())).collect()
}
