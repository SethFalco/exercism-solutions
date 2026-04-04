/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let bytes = isbn.as_bytes();

    if bytes.contains(&b'X') && bytes.last().unwrap() != &b'X' {
        return false;
    }

    let nums: Vec<u32> = isbn.as_bytes().iter()
        .filter_map(|&digit| {
            if digit == b'X' {
                return Some(10);
            }

            if !digit.is_ascii_digit() {
                return None;
            }

            char::from(digit).to_digit(10)
        })
        .collect();

    if nums.len() != 10 {
        return false;
    }

    let sum = nums.iter()
        .enumerate()
        .fold(0, |acc, (i, digit)| {
            acc + digit * (nums.len() - i) as u32
        });

    sum % 11 == 0
}
