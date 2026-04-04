#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(array_a: &[T], array_b: &[T]) -> Comparison {
    if array_a == array_b {
        return Comparison::Equal
    }

    let a_len = array_a.len();
    let b_len = array_b.len();

    if a_len == 0 || array_b.windows(a_len).any(|window| window == array_a) {
        return Comparison::Sublist
    }

    if b_len == 0 || array_a.windows(b_len).any(|window| window == array_b) {
        return Comparison::Superlist
    }

    Comparison::Unequal
}
