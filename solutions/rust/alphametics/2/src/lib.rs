use std::collections::{HashMap, HashSet};
use itertools::Itertools;

static DIGITS: [u8; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0];

#[derive(Debug, PartialEq, Clone)]
struct Alphametic {
    operands: Vec<Vec<char>>,
    result: Vec<char>,
}

/// Only solves equations where all characters represent a unique value, the
/// answer has no leading zeroes, and only handles addition.
pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let alphametic = parse_alphametic(input)?;
    let (unique, leading) = classify_chars(&alphametic);

    for iter in DIGITS.iter().permutations(unique.len()) {
        let opt_map = unique.iter()
            .zip(iter)
            .try_fold(HashMap::new(), |mut acc, (ch, val)| {
                if val == &0 && leading.contains(ch) {
                    return None;
                }

                acc.insert(**ch, *val);
                Some(acc)
            });

        if opt_map.is_none() {
            continue;
        }

        let map = opt_map.unwrap();

        let operands: u64 = alphametic.operands.iter()
            .map(|chars| chars_to_nums(chars, &map))
            .sum();

        let result: u64 = chars_to_nums(&alphametic.result, &map);

        if operands == result {
            return Some(map);
        }
    }

    None
}

fn parse_alphametic(body: &str) -> Option<Alphametic> {
    let split: Vec<&str> = body.split("==").collect();

    if split.len() != 2 {
        return None;
    }

    let mut operands = Vec::new();

    for str in split[0].split_ascii_whitespace() {
        let chars: Vec<char> = str.as_bytes().iter()
            .filter(|x| x.is_ascii_uppercase())
            .map(|&x| char::from(x))
            .collect();

        if !chars.is_empty() {
            operands.push(chars);
        }
    }

    let result: Vec<char> = split[1].trim().as_bytes().iter()
        .map(|&x| char::from(x))
        .collect();

    Some(Alphametic { operands, result })
}

fn classify_chars(alphametic: &Alphametic) -> (HashSet<&char>, HashSet<&char>) {
    let mut unique = HashSet::new();
    let mut leading = HashSet::new();

    for operand in &alphametic.operands {
        leading.insert(&operand[0]);

        for ch in operand {
            unique.insert(ch);
        }
    }

    leading.insert(&&alphametic.result[0]);

    for ch in &alphametic.result {
        unique.insert(ch);
    }

    (unique, leading)
}

fn chars_to_nums(chars: &Vec<char>, map: &HashMap<char, u8>) -> u64 {
    chars.iter().fold(0, |acc, ch| {
        acc * 10 + *map.get(ch).unwrap() as u64
    })
}
