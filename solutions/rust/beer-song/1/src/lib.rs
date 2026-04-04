pub fn verse(n: u32) -> String {
    let result = match n {
        0 => "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_owned(),
        1 => "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_owned(),
        2 => "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n".to_owned(),
        _ => format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", n, n, n - 1)
    };

    result
}

pub fn sing(start: u32, end: u32) -> String {
    let mut result = Vec::with_capacity((start - end + 1) as usize);

    for i in (end..=start).rev() {
        result.push(verse(i));
    }

    result.join("\n")
}
