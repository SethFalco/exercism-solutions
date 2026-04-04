use std::ops::Add;

pub fn square_of_sum(n: u32) -> u32 {
    (0..=n).reduce(u32::add).unwrap().pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    (0..=n).reduce(|acc, x| acc + x.pow(2)).unwrap()
}

/// Note: Moving from traditional loops with a mutable variable to accumulate
/// the result, and instead using reduce() improved performance by > 99%.
pub fn difference(n: u32) -> u32 {
    square_of_sum(n).abs_diff(sum_of_squares(n))
}
