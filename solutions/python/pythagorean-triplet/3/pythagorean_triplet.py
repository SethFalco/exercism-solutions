import math
from itertools import combinations


MIN_PYTHAGOREAN_VALUE = 3
"""
The lowest possible Pythagorean triple value.
There's no need to check values below this.
"""


def triplets_with_sum(number: int) -> list[list[int]]:
    cap = int(number / 2)
    results = []

    for a, b in combinations(range(MIN_PYTHAGOREAN_VALUE, cap), 2):
        c = math.sqrt(a ** 2 + b ** 2)

        if a + b + c == number:
            results.append([a, b, c])

    return results
