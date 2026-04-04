"""Functions which helps the locomotive engineer to keep track of the train."""


def get_list_of_wagons(*args: tuple[str]) -> list:
    """
    Return a list of wagons.

    :param: arbitrary number of wagons.
    :return: list of wagons.
    """
    return list(args)


def fix_list_of_wagons(each_wagons_id: list, missing_wagons: list) -> list:
    """
    Fix the list of wagons.

    :parm each_wagons_id: the list of wagons.
    :parm missing_wagons: the list of missing wagons.
    :return: list of wagons.
    """
    first, second, third, *rest = each_wagons_id
    return [third, *missing_wagons, *rest, first, second]


def add_missing_stops(route: dict[str, str], **kwargs: dict) -> dict:
    """
    Add missing stops to route dict.

    :param route: the dict of routing information.
    :param: arbitrary number of stops.
    :return: updated route dictionary.
    """
    return {**route, "stops": list(kwargs.values())}


def extend_route_information(route: dict, more_route_information: dict) -> dict:
    """
    Extend route information with more_route_information.

    :param route: the route information.
    :param more_route_information: extra route information.
    :return: extended route information.
    """
    return {**route, **more_route_information}


def fix_wagon_depot(wagons_rows: list[list[tuple]]) -> list[list[tuple]]:
    """
    Fix the list of rows of wagons.

    :param wagons_rows: the list of rows of wagons.
    :return: list of rows of wagons.
    """
    return list(map(list, zip(*wagons_rows)))
