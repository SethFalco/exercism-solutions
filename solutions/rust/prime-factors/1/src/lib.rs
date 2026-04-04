pub fn factors(mut n: u64) -> Vec<u64> {
    let mut vec = Vec::new();

    for progress in 2u64..=n {
        if n == 1 {
            return vec;
        }

        while n % progress == 0 {
            vec.push(progress);
            n /= progress;
        }
    }

    vec
}
