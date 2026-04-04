pub fn reply(body: &str) -> &str {
    let normalized = body.trim();

    if normalized.is_empty() {
        return "Fine. Be that way!";
    }

    let has_alpha_chars = has_ascii_alpha_chars(normalized);
    let is_uppercase = is_ascii_uppercase(normalized);
    let is_question = is_question(normalized);

    if has_alpha_chars && is_uppercase && is_question {
        return "Calm down, I know what I'm doing!";
    }

    if is_question {
        return "Sure.";
    }

    if has_alpha_chars && is_uppercase {
        return "Whoa, chill out!";
    }

    "Whatever."
}

/// Returns if the string contains any ASCII alpha characters.
fn has_ascii_alpha_chars(body: &str) -> bool {
    body.bytes().any(|x| x.is_ascii_alphabetic())
}

/// Returns if all ASCII characters in the string are uppercase.
/// Ignores non-ASCII characters.
fn is_ascii_uppercase(body: &str) -> bool {
    body.to_uppercase() == body
}

/// Returns if the sentence is a question.
fn is_question(body: &str) -> bool {
    body.ends_with("?")
}
