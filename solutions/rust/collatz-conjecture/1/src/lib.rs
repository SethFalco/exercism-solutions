pub fn collatz(mut n: u64) -> Option<u64> {
    match n {
        0 => return None,
        1 => return Some(0),
        _ => {
            for i in 1.. {
                if is_even(n) {
                    n /= 2;
                } else {
                    n = n.checked_mul(3)?;
                    n = n.checked_add(1)?;
                }

                if n == 1 {
                    return Some(i);
                }
            }
        }
    }

    None
}

fn is_even(x: u64)  -> bool{
    x.trailing_ones() == 0
}
