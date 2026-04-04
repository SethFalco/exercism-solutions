pub fn square(s: u32) -> u64 {
    if s < 1 || s > 64 {
        panic!("Square must be between 1 and 64");
    }

    2u64.pow(s - 1)
}

/// Note: While I don't do this, it'd make more sense to return a static
/// integer here rather than calculate it every time. It's not like the result
/// will change in runtime.
pub fn total() -> u64 {
    (1..=64).reduce(|acc, x| acc + square(x as u32)).unwrap()
}
