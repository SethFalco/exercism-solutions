from enum import Enum
from typing import Iterable


class Flag(Enum):
    PREPEND_LINE_NUMBER = "n"
    SHOW_PATH_ONLY = "l"
    CASE_INSENSITIVE = "i"
    INVERT = "v"
    WHOLE_LINE_ONLY = "x"


def grep(pattern: str, flags_str: str, paths: Iterable[str]) -> str:
    flags = parse_flags(flags_str)
    casefold = Flag.CASE_INSENSITIVE in flags

    if casefold:
        pattern = pattern.casefold()

    matches = []
    whole_line = Flag.WHOLE_LINE_ONLY in flags
    invert = Flag.INVERT in flags

    for path in paths:
        with open(path, "r") as file:
            line_number = 0

            while line := file.readline():
                line = line.strip()
                line_number += 1
                effective_line = line if not casefold else line.casefold()

                if is_match(effective_line, pattern, whole_line, invert):
                    matches.append((path, line_number, line))

    result = []
    path_only = Flag.SHOW_PATH_ONLY in flags
    show_file_names = len(paths) > 1

    for path, line_number, line in matches:
        row = []

        if show_file_names or path_only:
            row.append(path)

        if not path_only and Flag.PREPEND_LINE_NUMBER in flags:
            row.append(str(line_number))

        if not path_only:
            row.append(line)

        to_append = ":".join(row) + "\n"

        if to_append not in result:
            result.append(to_append)

    return "".join(result)


def parse_flags(flags_str: str) -> set[Flag]:
    split = list(map(lambda str: str[1], flags_str.split()))
    return {flag for flag in Flag if flag.value in split}


def is_match(body: str, pattern: str, whole_line: bool, invert: bool) -> bool:
    match = pattern in body if not whole_line else pattern == body
    return match if not invert else not match
