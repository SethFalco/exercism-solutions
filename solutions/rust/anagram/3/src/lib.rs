use std::collections::{HashSet, HashMap};

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    possible_anagrams.iter().fold(HashSet::new(), |mut accum, item| {
        if word.len() != item.len() {
            return accum;
        }

        let mut map = HashMap::new();
        let item_lower = item.to_lowercase();

        for c in item_lower.chars() {
            *map.entry(c).or_insert(0) += 1;
        }

        let word_lower = word.to_lowercase();

        for c in word_lower.chars() {
            let opt_value = map.get(&c);

            if opt_value.is_none() {
                return accum;
            }

            let value = opt_value.unwrap();

            if *value == 0 {
                return accum;
            }

            map.insert(c, value - 1);
        }

        if !map.values().any(|&x| x != 0) && item_lower != word_lower {
            accum.insert(item);
        }

        accum
    })
}
