/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut valid_chars = 0;

    let opt_result = code.as_bytes().iter().rev()
        .filter(|&&x| x != b' ')
        .enumerate()
        .try_fold(0, |acc, (i, &x)| {
            if !x.is_ascii_digit() {
                return None;
            }

            valid_chars += 1;

            let digit = char::from(x).to_digit(10).unwrap();

            if i.trailing_ones() == 0 || digit == 9 {
                return Some(acc + digit);
            }

            let double = digit * 2;

            if double < 9 {
                return Some(acc + double);
            }

            return Some(acc + double - 9);
        });

    if opt_result.is_none() || valid_chars <= 1 {
        return false;
    }

    opt_result.unwrap() % 10 == 0
}
