pub fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
    let quotient = dividend / divisor;
    let remainder = dividend - (quotient * divisor);
    (quotient, remainder)
}

pub fn evens<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    iter.enumerate()
        .filter_map(
            |(i, item)| if i.trailing_zeros() > 0 { Some(item) } else { Option::None }
        )
}

pub struct Position(pub i16, pub i16);

impl Position {

    pub fn manhattan(&self) -> i16 {
        self.0.abs() + self.1.abs()
    }
}
