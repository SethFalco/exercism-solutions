#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }

    if num == 1 {
        return Some(Classification::Deficient);
    }

    let result = (1..=(num as f64).sqrt() as u64).reduce(|acc, x| {
        let factor = num / x;

        if x * factor != num {
            return acc;
        }

        if x == factor {
            return acc + x;
        }

        acc + x + factor
    })?;

    if result == num {
        return Some(Classification::Perfect);
    }

    if result > num {
        return Some(Classification::Abundant);
    }

    Some(Classification::Deficient)
}
