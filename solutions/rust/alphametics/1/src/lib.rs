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

    let mut map = HashMap::new();

    'outer: for iter in DIGITS.iter().permutations(unique.len()) {
        for (char, &val) in unique.iter().zip(iter) {
            if val == 0 && leading.contains(char) {
                continue 'outer;
            }

            map.insert(**char, val);
        }

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

        for char in operand {
            unique.insert(char);
        }
    }

    leading.insert(&&alphametic.result[0]);

    for char in &alphametic.result {
        unique.insert(char);
    }

    (unique, leading)
}

fn chars_to_nums(chars: &Vec<char>, map: &HashMap<char, u8>) -> u64 {
    let mut result: u64 = 0;

    for char in chars {
        let digit = map.get(char).unwrap();
        print!("{} {} {}\n", char, result, digit);
        result = result * 10 + *digit as u64;
    }

    result
}

#[test]
fn test_parse_alphametic() {
    let operands = vec![vec!['I'], vec!['B', 'B']];
    let result = vec!['I', 'L', 'L'];

    let actual = parse_alphametic("I + BB == ILL").unwrap();
    let expected = Alphametic { operands, result };

    assert_eq!(actual, expected);
}

#[test]
fn test_parse_alphametic_2() {
    let operands = vec![vec!['A', 'C', 'A'], vec!['D', 'D']];
    let result = vec!['B', 'D'];

    let actual = parse_alphametic("ACA + DD == BD").unwrap();
    let expected = Alphametic { operands, result };

    assert_eq!(actual, expected);
}

#[test]
fn test_classify_chars() {
    let operands = vec![vec!['I'], vec!['B', 'B']];
    let result = vec!['I', 'L', 'L'];
    let alphametic = Alphametic { operands, result };

    let actual = classify_chars(&alphametic);
    let expected = (HashSet::from([&'I', &'B', &'L']), HashSet::from([&'I',&'B']));

    assert_eq!(actual, expected);
}

#[test]
fn test_classify_chars_2() {
    let operands = vec![vec!['A', 'C', 'A'], vec!['D', 'D']];
    let result = vec!['B', 'D'];
    let alphametic = Alphametic { operands, result };

    let actual = classify_chars(&alphametic);
    let expected = (HashSet::from([&'A', &'C', &'D', &'B']), HashSet::from([&'A', &'D', &'B']));

    assert_eq!(actual, expected);
}

#[test]
fn test_chars_to_nums() {
    let actual = chars_to_nums(&vec!['I', 'L', 'L'], &HashMap::from([('I', 1u8), ('L', 0u8)]));
    let expected = 100;

    assert_eq!(actual, expected);
}
