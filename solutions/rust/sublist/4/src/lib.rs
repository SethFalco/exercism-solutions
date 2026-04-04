#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list == _second_list {
        return Comparison::Equal
    }

    let first_list_len = _first_list.len();
    let second_list_len = _second_list.len();

    if first_list_len == 0 || _second_list.windows(first_list_len).any(|x| x == _first_list) {
        return Comparison::Sublist
    }

    if second_list_len == 0 || _first_list.windows(second_list_len).any(|x| x == _second_list) {
        return Comparison::Superlist
    }

    Comparison::Unequal
}
