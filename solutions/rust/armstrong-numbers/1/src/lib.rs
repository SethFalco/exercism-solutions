pub fn is_armstrong_number(num: u32) -> bool {
    let digits = get_digits(num);
    let result = digits.iter().fold(0, |acc, x| {
        acc + x.pow(digits.len() as u32)
    });

    num == result
}

/// Note: While I don't do this here, it seems for numbers >= 5 digits, doing
/// to_string() -> as_bytes() -> char::from() -> to_digit(10), is faster.
fn get_digits(num: u32) -> Vec<u32> {
    if num == 0 {
        return vec![0];
    }

    let mut remaining = num;
    let mut result = Vec::new();

    while remaining != 0 {
        let digit = remaining % 10;
        remaining /= 10;
        result.push(digit);
    }

    return result;
}
