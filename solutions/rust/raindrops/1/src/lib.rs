/// Note: While I don't do this, it would be faster to handle n % for all combinations
/// of 3, 5, and 7. So n % 105, 35, 21, and 15 respectively.
pub fn raindrops(n: u32) -> String {
    let result = [(3, "Pling"), (5, "Plang"), (7, "Plong")]
        .iter()
        .fold(String::new(), |acc, (x, str)| {
            if n % x == 0 {
                return acc + str;
            }

            return acc;
        });

    if result.is_empty() {
        return n.to_string();
    }

    result
}
