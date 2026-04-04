def reverse(text: str) -> str:
    if text == None:
        raise ValueError("text must not be None")

    return text[::-1]
