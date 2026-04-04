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
    if from_base <= 1 {
        return Err(Error::InvalidInputBase);
    }

    if to_base <= 1  {
        return Err(Error::InvalidOutputBase);
    }

    let mut value = 0;

    for (i, &digit) in number.iter().rev().enumerate() {
        if digit >= from_base {
            return Err(Error::InvalidDigit(digit));
        }

        value += digit * from_base.pow(i as u32);
    }

    if value < to_base {
        return Result::Ok(vec![value]);
    }

    let mut result = Vec::new();

    while value != 0 {
        let div = value / to_base;
        result.insert(0, value - div * to_base);
        value = div;
    }

    Result::Ok(result)
}
