"""
Functions to help play and score a game of blackjack.

How to play blackjack:    https://bicyclecards.com/how-to-play/blackjack/
"Standard" playing cards: https://en.wikipedia.org/wiki/Standard_52-card_deck
"""


__card_map__ = {
    "J": 10,
    "Q": 10,
    "K": 10,
    "10": 10,
    "9": 9,
    "8": 8,
    "7": 7,
    "6": 6,
    "5": 5,
    "4": 4,
    "3": 3,
    "2": 2,
    "A": 1
}


def value_of_card(card: str) -> int:
    """
    Determine the scoring value of a card.

    :param card: given card.
    :return: value of a given card.  See below for values.

    1.  'J', 'Q', or 'K' (otherwise known as "face cards") = 10
    2.  'A' (ace card) = 1
    3.  '2' - '10' = numerical value.
    """
    if card in __card_map__:
        return __card_map__[card]

    raise ValueError("Invalid card name given.")


def higher_card(card_a: str, card_b: str) -> str | tuple:
    """
    Determine which card has a higher value in the hand.

    :param card_a, card_b: cards dealt in hand.  See below for values.
    :return: resulting Tuple contains both cards if they are of equal value.

    1.  'J', 'Q', or 'K' (otherwise known as "face cards") = 10
    2.  'A' (ace card) = 1
    3.  '2' - '10' = numerical value.
    """
    value_a = value_of_card(card_a)
    value_b = value_of_card(card_b)

    if value_a == value_b:
        return (card_a, card_b)

    if value_a > value_b:
        return card_a

    return card_b


def value_of_ace(card_a: str, card_b: str) -> int:
    """
    Calculate the most advantageous value for the ace card.

    :param card_a, card_b: card dealt. See below for values.
    :return: either 1 or 11 value of the upcoming ace card.

    1.  'J', 'Q', or 'K' (otherwise known as "face cards") = 10
    2.  'A' (ace card) = 11 (if already in hand)
    3.  '2' - '10' = numerical value.
    """
    value = value_of_card(card_a) + value_of_card(card_b)

    if "A" in [card_a, card_b]:
        value += 10

    return 11 if value <= 10 else 1


def is_blackjack(card_a: str, card_b: str) -> bool:
    """
    Determine if the hand is a 'natural' or 'blackjack'.

    :param card_a, card_b: card dealt. See below for values.
    :return: is the hand is a blackjack (two cards worth 21).

    1.  'J', 'Q', or 'K' (otherwise known as "face cards") = 10
    2.  'A' (ace card) = 11 (if already in hand)
    3.  '2' - '10' = numerical value.
    """
    if card_a == "A" and value_of_card(card_b) == 10:
        return True

    if value_of_card(card_a) == 10 and card_b == "A":
        return True

    return False


def can_split_pairs(card_a: str, card_b: str) -> bool:
    """
    Determine if a player can split their hand into two hands.

    :param card_a, card_b: cards dealt.
    :return: can the hand be split into two pairs? (i.e. cards are of the same value).
    """
    return value_of_card(card_a) == value_of_card(card_b)


def can_double_down(card_a: str, card_b: str) -> bool:
    """
    Determine if a blackjack player can place a double down bet.

    :param card_a, card_b: first and second cards in hand.
    :return: can the hand can be doubled down? (i.e. totals 9, 10 or 11 points).
    """
    return 9 <= value_of_card(card_a) + value_of_card(card_b) <= 11
