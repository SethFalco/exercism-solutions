pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len == 0 {
        return vec![String::new(); digits.len() + 1];
    }

    let chars: Vec<char> = digits.chars().collect();

    chars.windows(len).map(|x| {
        let word: String = x.iter().collect();
        word
    }).collect()
}
