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

    if _first_list.len() == 0 || sublist_contains(_second_list, _first_list) {
        return Comparison::Sublist
    }

    if _second_list.len() == 0 || sublist_contains(_first_list, _second_list) {
        return Comparison::Superlist
    }

    Comparison::Unequal
}

/// Check if list a contains sublist b.
pub fn sublist_contains<T: PartialEq>(_list: &[T], _sublist: &[T]) -> bool {
    let b_first_value = _sublist.get(0).unwrap();
    let mut effective_a = _list.clone();

    loop {
        if effective_a.starts_with(_sublist) {
            return true
        }

        let opt_position = effective_a.iter().position(|x| x == b_first_value);

        if opt_position.is_none() {
            return false
        }

        let position = opt_position.unwrap();

        if position != 0 {
            effective_a = effective_a.split_at(position).1;
        } else {
            effective_a = effective_a.split_at(1).1;
        }
    }
}
