def exchange_money(budget: float, exchange_rate: float) -> float:
    """
    :param budget: Amount of money you are planning to exchange.
    :param exchange_rate: Unit value of the foreign currency.
    :return: Exchanged value of the foreign currency you can receive.
    """
    return budget / exchange_rate


def get_change(budget: float, exchanging_value: int) -> float:
    """
    :param budget: Amount of money you own.
    :param exchanging_value: Amount of your money you want to exchange now.
    :return: Amount left of your starting currency after exchanging.
    """
    return budget - exchanging_value


def get_value_of_bills(denomination: int, number_of_bills: int) -> int:
    """
    :param denomination: The value of a bill.
    :param number_of_bills: Amount of bills you received.
    :return: Total value of bills you now have.
    """
    return denomination * number_of_bills


def get_number_of_bills(budget: float, denomination: int) -> int:
    """
    :param budget: Amount of money you are planning to exchange.
    :param denomination: Value of a single bill.
    :return: Number of bills after exchanging all your money.
    """
    return int(budget / denomination)


def get_leftover_of_bills(budget: float, denomination: int) -> float:
    """
    :param budget: Amount of money you are planning to exchange.
    :param denomination: Value of a single bill.
    :return: Leftover amount that cannot be exchanged given the current denomination.
    """
    return budget % denomination


def exchangeable_value(budget: float, exchange_rate: float, spread: int, denomination: int) -> int:
    """
    :param budget: Amount of money you are planning to exchange.
    :param exchange_rate: Unit value of the foreign currency.
    :param spread: Percentage that is taken as an exchange fee.
    :param denomination: Value of a single bill.
    :return: Maximum value you can get.
    """
    effective_exchange_rate = exchange_rate * (1 + (spread / 100))
    exchanged = exchange_money(budget, effective_exchange_rate)
    bills = get_number_of_bills(exchanged, denomination)
    return get_value_of_bills(denomination, bills)
