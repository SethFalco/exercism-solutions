#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        let str = value.to_string();
        let bytes = str.as_bytes();
        let mut rev = bytes.clone().to_owned();
        rev.reverse();

        if bytes != rev {
            return None;
        }

        Some(Self { 0: value })
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut min_result: Option<Palindrome> = None;
    let mut max_result: Option<Palindrome> = None;

    'outer: for i in min..=max {
        for ii in min..=max {
            let palindrome = Palindrome::new(i * ii);

            if palindrome.is_none() {
                continue;
            }

            min_result = palindrome;
            break 'outer;
        }
    }

    for i in (min..=max).rev() {
        for ii in (min..=max).rev() {
            let palindrome = Palindrome::new(i * ii);

            if palindrome.is_none() {
                continue;
            }

            if max_result.is_none() {
                max_result = palindrome;
            } else if max_result.unwrap().0 < palindrome.unwrap().0 {
                max_result = palindrome;
            }
        }
    }

    Some((min_result?, max_result?))
}
