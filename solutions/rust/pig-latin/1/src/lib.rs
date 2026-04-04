use std::collections::VecDeque;

const VOWELS: [&'static str; 7] = [
    "a",
    "e",
    "i",
    "o",
    "u",
    "xr",
    "yt"
];

pub fn translate(input: &str) -> String {
    let words: Vec<String> = input.split(" ").map(|word| {
        let vowel_start = VOWELS.iter().any(|v| word.starts_with(v));
        let mut bytes: VecDeque<u8> = word.as_bytes().iter().cloned().collect();

        if !vowel_start {
            let mut cluster_len = 1;

            for i in 1..bytes.len() {
                if VOWELS.iter().any(|&v| v == char::from(bytes[i]).to_string()) {
                    break;
                }

                if bytes[i] == b'y' {
                    break;
                }

                cluster_len += 1;
            }

            if bytes[cluster_len - 1] == b'q' && bytes[cluster_len] == b'u' {
                cluster_len += 1;
            }

            for _ in 0..cluster_len {
                let char = bytes.pop_front().unwrap();
                bytes.push_back(char);
            }
        }

        bytes.push_back(b'a');
        bytes.push_back(b'y');

        String::from_utf8(bytes.iter().cloned().collect()).unwrap()
    }).collect();

    words.join(" ")
}
