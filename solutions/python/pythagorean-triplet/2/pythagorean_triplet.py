import math
from itertools import combinations


def triplets_with_sum(number: int) -> list[list[int]]:
    cap = int(number / 2)
    results = []

    for a, b in combinations(range(2, cap), 2):
        c = math.sqrt(a ** 2 + b ** 2)

        if a + b + c == number:
            results.append([a, b, int(c)])

    return results
