MIN_PYTHAGOREAN_VALUE = 3
"""
The lowest possible Pythagorean triple value.
There's no need to check values below this.
"""


def triplets_with_sum(number: int) -> list[list[int]]:
    """
    A naive bruteforce algorithm to compute Pythagorean triples.
    """
    results = []
    cap = number >> 1
    a = MIN_PYTHAGOREAN_VALUE

    while a < cap:
        b = a + 1
        while b < cap:
            c = (a ** 2 + b ** 2) ** 0.5

            if a + b + c == number:
                results.append([a, b, c])
                cap = b
                break
            else:
                b += 1
        a += 1

    return results
