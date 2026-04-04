EXPECTED_BAKE_TIME = 40


PREPARATION_TIME = 2
"""
The time it takes to prepare a single layer of lasagna.
"""


def bake_time_remaining(elapsed_bake_time: int) -> int:
    """
    Calculate the bake time remaining.

    :param elapsed_bake_time: Baking time already elapsed.
    :return: Remaining bake time in minutes.
    """
    return EXPECTED_BAKE_TIME - elapsed_bake_time


def preparation_time_in_minutes(number_of_layers: int) -> int:
    """
    Calculates the time it would take to prepare a lasagna.

    :param number_of_layers: Total number of layers the lasagna will have.
    :return: The time in minutes it will take to prep.
    """
    return PREPARATION_TIME * number_of_layers


def elapsed_time_in_minutes(number_of_layers: int, elapsed_bake_time: int) -> int:
    """
    Calculates the elapsed time so far in baking a lasagna. Assumes that you
    have already prepared all layers.

    :param number_of_layers: Total number of layers the lasagna will have.
    :return: Total time that has elapsed, including prep time.
    """
    return preparation_time_in_minutes(number_of_layers) + elapsed_bake_time
