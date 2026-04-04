#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

/// Convert a number between two bases.
///
/// https://en.wikipedia.org/wiki/Positional_notation#Base_conversion
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    }

    if to_base < 2  {
        return Err(Error::InvalidOutputBase);
    }

    let mut value: u32 = 0;

    for &digit in number {
        if digit >= from_base {
            return Err(Error::InvalidDigit(digit));
        }

        value = value * from_base + digit;
    }

    if value < to_base {
        return Result::Ok(vec![value as u32]);
    }

    let mut result = Vec::new();

    while value != 0 {
        let div = value / to_base;
        result.insert(0, value - div * to_base);
        value = div;
    }

    Result::Ok(result)
}
