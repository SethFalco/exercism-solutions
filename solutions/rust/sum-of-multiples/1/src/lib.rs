pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (0..limit).reduce(|acc, x| {
        if factors.iter().filter(|&&y| y != 0).any(|y| x % y == 0) {
            return acc + x;
        }

        acc
    }).unwrap_or(0)
}
