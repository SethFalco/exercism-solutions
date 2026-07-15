defmodule BoutiqueSuggestions do
  @spec get_combinations([any()], [any()], [{:maximum_price, number()}]) :: any()
  def get_combinations(tops, bottoms, options \\ []) do
    maximum_price = Keyword.get(options, :maximum_price, 100.00)

    for t <- tops,
        b <- bottoms,
        t.base_color !== b.base_color,
        t.price + b.price <= maximum_price do
      {t, b}
    end
  end
end
