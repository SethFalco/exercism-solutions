from string import ascii_lowercase


def is_pangram(sentence: str):
    return set(ascii_lowercase).issubset(sentence.lower())
