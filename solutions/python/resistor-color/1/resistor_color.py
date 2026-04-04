color_map = {
    "black": 0,
    "brown": 1,
    "red": 2,
    "orange": 3,
    "yellow": 4,
    "green": 5,
    "blue": 6,
    "violet": 7,
    "grey": 8,
    "white": 9
}
"""
See: https://en.wikipedia.org/wiki/Electronic_color_code
"""

def color_code(color: str) -> int:
    return color_map[color]


def colors() -> list[str]:
    return list(color_map)
