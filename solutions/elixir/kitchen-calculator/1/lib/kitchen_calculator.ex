defmodule KitchenCalculator do
  def get_volume(volume_pair), do: elem(volume_pair, 1)

  def to_milliliter(volume_pair) do
    volume = get_volume(volume_pair)

    case elem(volume_pair, 0) do
      :cup -> { :milliliter, volume * 240 }
      :fluid_ounce -> { :milliliter, volume * 30 }
      :teaspoon -> { :milliliter, volume * 5 }
      :tablespoon -> { :milliliter, volume * 15 }
      :milliliter -> { :milliliter, volume * 1 }
    end
  end

  def from_milliliter(volume_pair, unit) do
    milliliters = get_volume(to_milliliter(volume_pair))
    { unit, milliliters / get_volume(to_milliliter({ unit, 1 })) }
  end

  def convert(volume_pair, unit) do
    volume_pair |> to_milliliter() |> from_milliliter(unit)
  end
end
