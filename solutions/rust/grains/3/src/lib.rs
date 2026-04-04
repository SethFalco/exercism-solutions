pub fn square(x: u32) -> u64 {
    if x < 1 || x > 64 {
        panic!("Square must be between 1 and 64");
    }

    2u64.pow(x - 1)
}

/// Note: While I don't do this, it'd make more sense to return a static
/// integer here rather than calculate it every time. It's not like the result
/// will change in runtime.
pub fn total() -> u64 {
    (1..=64).map(grains::square).sum()
}
