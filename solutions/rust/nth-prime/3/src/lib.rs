pub fn nth(n: u32) -> u32 {
    let mut progress = 0;
    let mut result = 2;

    loop {
        if progress == n {
            return result;
        }

        result += 1;

        let end = (result as f32).sqrt() as u32;

        if (2..=end).any(|x| result % x == 0) {
            continue;
        }

        progress += 1;
    }
}
