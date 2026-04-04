pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<u8> = Vec::new();

    for &byte in string.as_bytes() {
        if byte == b'[' || byte == b'(' || byte == b'{'  {
            stack.push(byte);
            continue;
        }

        if byte == b']' {
            let opt_open = stack.pop();

            if opt_open.is_none() || opt_open.unwrap() != b'[' {
                return false;
            }
        }

        if byte == b')' {
            let opt_open = stack.pop();

            if opt_open.is_none() || opt_open.unwrap() != b'(' {
                return false;
            }
        }

        if byte == b'}' {
            let opt_open = stack.pop();

            if opt_open.is_none() || opt_open.unwrap() != b'{' {
                return false;
            }
        }
    }

    stack.is_empty()
}
