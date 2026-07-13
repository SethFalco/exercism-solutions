defmodule BoutiqueInventory do
  @spec sort_by_price([map()]) :: [map()]
  def sort_by_price(inventory) do
    Enum.sort_by(inventory, &(&1.price))
  end

  @spec with_missing_price([map()]) :: [map()]
  def with_missing_price(inventory) do
    Enum.filter(inventory, &(&1.price === nil))
  end

  @spec update_names([map()], String.t(), String.t()) :: [map()]
  def update_names(inventory, old_word, new_word) do
    Enum.map(inventory, fn item ->
      Map.put(item, :name, String.replace(item.name, old_word, new_word))
    end)
  end

  @spec increase_quantity(map(), integer()) :: map()
  def increase_quantity(item, count) do
    Map.put(
      item,
      :quantity_by_size,
      Map.new(item.quantity_by_size, fn {k, v} ->
        {k, v + count}
      end)
    )
  end

  @spec total_quantity(map()) :: integer()
  def total_quantity(item) do
    Enum.reduce(item.quantity_by_size, 0, fn {_, v}, acc -> acc + v end)
  end
end
