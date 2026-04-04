pub fn nth(n: u32) -> u32 {
    let mut progress = 0;
    let mut result = 2;

    loop {
        if progress == n {
            return result;
        }

        result += 1;

        if (2..=result / 2).any(|x| result % x == 0) {
            continue;
        }

        progress += 1;
    }
}
