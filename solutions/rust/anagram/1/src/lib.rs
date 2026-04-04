use std::collections::{HashSet, HashMap};

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    possible_anagrams.iter().fold(HashSet::new(), |mut accum, item| {
        let mut map = HashMap::new();

        for c in item.chars() {
            let key;

            if c.is_lowercase() {
                key = c.to_string();
            } else {
                key = c.to_lowercase().to_string();
            }

            *map.entry(key).or_insert(0) += 1;
        }

        for c in word.chars() {
            let key;

            if c.is_lowercase() {
                key = c.to_string();
            } else {
                key = c.to_lowercase().to_string();
            }

            let opt_value = map.get(&key);

            if opt_value.is_none() {
                return accum;
            }

            let value = opt_value.unwrap();

            if *value == 0 {
                return accum;
            }

            map.insert(key, value - 1);
        }

        if !map.values().any(|&x| x != 0) && item.to_lowercase() != word.to_lowercase() {
            accum.insert(item);
        }

        accum
    })
}
