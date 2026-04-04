/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    if code.as_bytes().iter().any(|&x| x != b' ' && !x.is_ascii_digit()) {
        return false;
    }

    let mut spaces = 0;
    let mut valid_chars = 0;

    let result = code.as_bytes().iter().rev().enumerate().fold(0, |acc, (i, &x)| {
        if x == b' ' {
            spaces += 1;
            return acc;
        }

        valid_chars += 1;

        let digit = char::from(x).to_digit(10).unwrap();

        if (i - spaces).trailing_ones() == 0 || digit == 9 {
            return acc + digit;
        }

        let double = digit * 2;

        if double < 9 {
            return acc + double;
        }

        return acc + double - 9;
    });

    if valid_chars <= 1 {
        return false;
    }

    result % 10 == 0
}
