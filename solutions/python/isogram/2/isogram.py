from string import ascii_lowercase


def is_isogram(body: str) -> bool:
    lower = body.casefold()

    for char in ascii_lowercase:
        if lower.count(char) > 1:
            return False

    return True
