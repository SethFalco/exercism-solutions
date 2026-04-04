/// ASCII value of a space ( ) character.
const SPACE: u8 = 32;

/// ASCII value of an asterisk (*) character.
const ASTERISK: u8 = 42;

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut result = vec![String::new(); minefield.len()];

    for (i, row) in minefield.iter().enumerate() {
        let row_bytes = row.as_bytes();

        for (ii, byte) in row_bytes.iter().enumerate() {
            if *byte != SPACE {
                result[i] += "*";
                continue;
            }

            let mut adjacent_bombs = 0;

            if i != 0 {
                let bytes = minefield.get(i - 1).unwrap().as_bytes();
                adjacent_bombs += check_row(bytes, ii);
            }

            if i != minefield.len() - 1 {
                let bytes = minefield.get(i + 1).unwrap().as_bytes();
                adjacent_bombs += check_row(bytes, ii);
            }

            if ii != 0 && *row_bytes.get(ii - 1).unwrap() == ASTERISK {
                adjacent_bombs += 1;
            }

            if ii != row.len() - 1 && *row_bytes.get(ii + 1).unwrap() == ASTERISK {
                adjacent_bombs += 1;
            }

            if adjacent_bombs == 0 {
                result[i] += " ";
            } else {
                result[i] += &adjacent_bombs.to_string();
            }
        }
    }

    result
}

fn check_row(row: &[u8], column: usize) -> u8 {
    let row_same_col = row[column];
    let mut result = 0;

    if row_same_col == ASTERISK {
        result += 1;
    }

    if column != 0 && *row.get(column - 1).unwrap() == ASTERISK {
        result += 1;
    }

    if column != row.len() - 1 && *row.get(column + 1).unwrap() == ASTERISK {
        result += 1;
    }

    result
}
