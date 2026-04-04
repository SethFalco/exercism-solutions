use std::collections::HashMap;

const NUCLEOTIDES: [char; 4] = ['A', 'C', 'G', 'T'];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !NUCLEOTIDES.contains(&nucleotide) {
        return Err(nucleotide);
    }

    let mut count = 0;

    for &d in dna.as_bytes() {
        let c = char::from(d);

        if !NUCLEOTIDES.contains(&c) {
            return Err(c);
        }

        if nucleotide == c {
            count += 1;
        }
    }

    return Ok(count);
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut map = HashMap::new();

    for &d in dna.as_bytes() {
        let c = char::from(d);

        if !NUCLEOTIDES.contains(&c) {
            return Err(c);
        }

        *map.entry(c).or_insert(0) += 1;
    }

    for n in NUCLEOTIDES {
        map.entry(n).or_insert(0);
    }

    Ok(map)
}
