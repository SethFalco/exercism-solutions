use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut map = HashMap::new();

    for word in magazine {
        if !note.contains(word) {
            continue;
        }

        *map.entry(word).or_insert(0) += 1;
    }

    for word in note {
        let opt_value = map.get(word);

        if opt_value.is_none() {
            return false;
        }

        let value = opt_value.unwrap();

        if *value == 0 {
            return false;
        }

        map.insert(word, value - 1);
    }

    true
}
