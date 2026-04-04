use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    h.iter().fold(BTreeMap::new(), |mut acc, (&val, chars)| {
        for &c in chars {
            acc.insert(c.to_ascii_lowercase(), val);
        }

        acc
    })
}
