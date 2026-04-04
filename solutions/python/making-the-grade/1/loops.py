"""Functions for organizing and calculating student exam scores."""


from functools import reduce


def round_scores(student_scores: list[float | int]) -> list[int]:
    """
    Round all provided student scores.

    :param student_scores: float or int of student exam scores.
    :return: student scores *rounded* to nearest integer value.
    """
    return map(lambda x: round(x), student_scores)


def count_failed_students(student_scores: list) -> int:
    """
    Count the number of failing students out of the group provided.

    :param student_scores: containing int student scores.
    :return: count of student scores at or below 40.
    """
    return reduce(lambda acc, val: acc + 1 if val <= 40 else acc, student_scores, 0)


def above_threshold(student_scores: list, threshold: int) -> list:
    """
    Determine how many of the provided student scores were 'the best' based on the provided threshold.

    :param student_scores: of integer scores.
    :param threshold: threshold to cross to be the "best" score.
    :return: of integer scores that are at or above the "best" threshold.
    """
    return list(filter(lambda val: val >= threshold, student_scores))


def letter_grades(highest: int) -> list:
    """
    Create a list of grade thresholds based on the provided highest grade.

    :param highest: value of highest exam score.
    :return: of lower threshold scores for each D-A letter grade interval.
            For example, where the highest score is 100, and failing is <= 40,
            The result would be [41, 56, 71, 86]:

            41 <= "D" <= 55
            56 <= "C" <= 70
            71 <= "B" <= 85
            86 <= "A" <= 100
    """
    if highest <= 40:
        raise ValueError("Highest score is under the failing threshold.")

    interval = round((highest - 41) / 4)
    return list(map(lambda x: 41 + interval * x, range(0, 4)))


def student_ranking(student_scores: list, student_names: list):
    """
    Organize the student's rank, name, and grade information in ascending order.

    :param student_scores: of scores in descending order.
    :param student_names: of string names by exam score in descending order.
    :return: of strings in format ["<rank>. <student name>: <score>"].
    """
    student_iter = enumerate(zip(student_scores, student_names))
    student_map = map(lambda info: f"{info[0] + 1}. {info[1][1]}: {info[1][0]}", student_iter)
    return list(student_map)


def perfect_score(student_info: list[str, int]) -> list[str, int]:
    """
    Create a list that contains the name and grade of the first student to make a perfect score on the exam.

    :param student_info: of [<student name>, <score>] lists.
    :return: first `[<student name>, 100]` or `[]` if no student score of 100 is found.
    """
    for info in student_info:
        if info[1] == 100:
            return info

    return []
