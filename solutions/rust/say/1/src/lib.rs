const DIGITS: [&'static str; 20] = [
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen"
];

const TENS: [&'static str; 8] = [
    "twenty",
    "thirty",
    "forty",
    "fifty",
    "sixty",
    "seventy",
    "eighty",
    "ninety"
];

const THOUSANDS: [&'static str; 6] = [
    "thousand",
    "million",
    "billion",
    "trillion",
    "quadrillion",
    "quintillion"
];

pub fn encode(mut n: u64) -> String {
    if n == 0 {
        return DIGITS[0].to_owned();
    }

    let mut thousands = Vec::new();

    while n != 0 {
        let division = n / 1000;
        let remainder = n - division * 1000;
        n = division;

        thousands.push(remainder);
    }

    let separated: Vec<String> = thousands.iter()
        .filter_map(|&x| {
            if x == 0 {
                return Some("".to_owned());
            }

            let mut str: Vec<String> = Vec::new();
            let hundreds = x / 100;

            if hundreds != 0 {
                str.push(DIGITS[hundreds as usize].to_owned() + " hundred");
            }

            let mut remainder = x - hundreds * 100;

            if remainder == 0 {
                return Some(str.join(" "));
            }

            if remainder < 20 {
                str.push(DIGITS[remainder as usize].to_owned());
                return Some(str.join(" "));
            }

            let tens = remainder / 10;
            remainder = remainder - tens * 10;

            if remainder == 0 {
                str.push(TENS[tens as usize - 2].to_owned());
            } else {
                str.push(TENS[tens as usize - 2].to_owned() + "-" + DIGITS[remainder as usize]);
            }

            Some(str.join(" "))
        })
        .collect();

    let result: Vec<String> = separated.iter().enumerate().filter_map(|(i, body)| {
        if body == "" {
            return None;
        }

        if i == 0 {
            return Some(body.to_owned());
        }

        Some(body.to_owned() + " " + &THOUSANDS[i - 1].to_owned())
    }).rev().collect();

    result.join(" ")
}
