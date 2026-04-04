"""Functions for compiling dishes and ingredients for a catering company."""


from functools import reduce
from sets_categories_data import (VEGAN, VEGETARIAN, KETO, PALEO, OMNIVORE, ALCOHOLS, SPECIAL_INGREDIENTS)


def clean_ingredients(dish_name: str, dish_ingredients: list[str]) -> tuple[str, set[str]]:
    """
    Remove duplicates from `dish_ingredients`.

    :param dish_name: containing the dish name.
    :param dish_ingredients: dish ingredients.
    :return: containing (dish_name, ingredient set).

    This function should return a `tuple` with the name of the dish as the first item,
    followed by the de-duped `set` of ingredients as the second item.
    """
    return (dish_name, set(dish_ingredients))


def check_drinks(drink_name: str, drink_ingredients: list[str]) -> str:
    """
    Append "Cocktail" (alcohol)  or "Mocktail" (no alcohol) to `drink_name`, based on `drink_ingredients`.

    :param drink_name: name of the drink.
    :param drink_ingredients: ingredients in the drink.
    :return: drink_name appended with "Mocktail" or "Cocktail".

    The function should return the name of the drink followed by "Mocktail" (non-alcoholic) and drink
    name followed by "Cocktail" (includes alcohol).
    """
    suffix = "Cocktail" if set(drink_ingredients) & ALCOHOLS else "Mocktail"
    return f"{drink_name} {suffix}"


def categorize_dish(dish_name: str, dish_ingredients: list[str]) -> str:
    """
    Categorize `dish_name` based on `dish_ingredients`.

    :param dish_name: dish to be categorized.
    :param dish_ingredients: ingredients for the dish.
    :return: the dish name appended with ": <CATEGORY>".

    This function should return a string with the `dish name: <CATEGORY>` (which meal category the dish belongs to).
    `<CATEGORY>` can be any one of  (VEGAN, VEGETARIAN, PALEO, KETO, or OMNIVORE).
    All dishes will "fit" into one of the categories imported from `sets_categories_data.py`
    """
    category = None

    if dish_ingredients <= VEGAN:
        category = "VEGAN"
    elif dish_ingredients <= VEGETARIAN:
        category = "VEGETARIAN"
    elif dish_ingredients <= PALEO:
        category = "PALEO"
    elif dish_ingredients <= KETO:
        category = "KETO"
    elif dish_ingredients <= OMNIVORE:
        category = "OMNIVORE"

    return f"{dish_name}: {category}" if category else dish_name


def tag_special_ingredients(dish: tuple[str, list[str] | set[str]]) -> tuple[str, set]:
    """
    Compare `dish` ingredients to `SPECIAL_INGREDIENTS`.

    :param dish: of (dish name, list of dish ingredients).
    :return: containing (dish name, dish special ingredients).

    Return the dish name followed by the `set` of ingredients that require a special note on the dish description.
    For the purposes of this exercise, all allergens or special ingredients that need to be tracked are in the
    SPECIAL_INGREDIENTS constant imported from `sets_categories_data.py`.
    """
    return (dish[0], set(dish[1]) & SPECIAL_INGREDIENTS)


def compile_ingredients(dishes: list[set[str]]) -> set[str]:
    """
    Create a master list of ingredients.

    :param dishes: of dish ingredient sets.
    :return: of ingredients compiled from `dishes`.

    This function should return a `set` of all ingredients from all listed dishes.
    """
    return reduce(lambda acc, val: acc | val, dishes, set())


def separate_appetizers(dishes: list[str], appetizers: list[str]) -> list[str]:
    """
    Determine which `dishes` are designated `appetizers` and remove them.

    :param dishes: of dish names.
    :param appetizers: of appetizer names.
    :return: of dish names that do not appear on appetizer list.

    The function should return the list of dish names with appetizer names removed.
    Either list could contain duplicates and may require de-duping.
    """
    return list(set(dishes) - set(appetizers))


def singleton_ingredients(dishes: list[set[str]], intersection: set[str]) -> set[str]:
    """
    Determine which `dishes` have a singleton ingredient (an ingredient that only appears once across dishes).

    :param dishes: of ingredient sets.
    :param intersection: can be one of `<CATEGORY>_INTERSECTIONS` constants imported from `sets_categories_data.py`.
    :return: containing singleton ingredients.

    Each dish is represented by a `set` of its ingredients.

    Each `<CATEGORY>_INTERSECTIONS` is an `intersection` of all dishes in the category. `<CATEGORY>` can be any one of:
        (VEGAN, VEGETARIAN, PALEO, KETO, or OMNIVORE).

    The function should return a `set` of ingredients that only appear in a single dish.
    """
    return reduce(lambda acc, val: acc | (val - intersection), dishes, set())
