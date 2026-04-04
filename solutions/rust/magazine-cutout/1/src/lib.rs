use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut map = HashMap::new();

    for word in magazine {
        if !note.contains(word) {
            continue;
        }

        map.insert(word, map.get(word).unwrap_or(&0) + 1);
    }

    for word in note {
        if !map.contains_key(word) {
            return false;
        }

        let new_value = map.get(word).unwrap() - 1;

        if new_value < 0 {
            return false;
        }

        map.insert(word, new_value);
    }

    true
}
