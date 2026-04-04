use std::fmt::format;

pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::new();
    }

    let mut result: Vec<String> = list.windows(2)
        .map(|words| {
            format!("For want of a {} the {} was lost.", words[0], words[1])
        })
        .collect();

    result.push(format!("And all for the want of a {}.", list.first().unwrap()));
    result.join("\n")
}
