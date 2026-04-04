"""Functions to keep track and alter inventory."""


def create_inventory(items: list) -> dict:
    """
    Create a dict that tracks the amount (count) of each element on the `items` list.

    :param items: list of items to create an inventory from.
    :return: the inventory dictionary.
    """
    return add_items({}, items)


def add_items(inventory: dict, items: list) -> dict:
    """
    Add or increment items in inventory using elements from the items `list`.

    :param inventory: dictionary of existing inventory.
    :param items: list of items to update the inventory with.
    :return: the inventory updated with the new items.
    """
    for item in items:
        inventory[item] = inventory.get(item, 0) + 1

    return inventory


def decrement_items(inventory: dict, items: list) -> dict:
    """
    Decrement items in inventory using elements from the `items` list.

    :param inventory: inventory dictionary.
    :param items: list of items to decrement from the inventory.
    :return: updated inventory with items decremented.
    """
    for item in items:
        count = inventory.get(item, 0)

        if count == 0:
            continue

        inventory[item] = count - 1

    return inventory


def remove_item(inventory: dict, item: str) -> dict:
    """
    Remove item from inventory if it matches `item` string.

    :param inventory: inventory dictionary.
    :param item: item to remove from the inventory.
    :return: updated inventory with item removed. Current inventory if item does not match.
    """
    if item in inventory:
        inventory.pop(item)

    return inventory


def list_inventory(inventory: dict) -> list[tuple]:
    """
    Create a list containing all (item_name, item_count) pairs in inventory.

    :param inventory: an inventory dictionary.
    :return: list of key, value pairs from the inventory dictionary.
    """
    return list([(k, v) for k, v in inventory.items() if v > 0])
