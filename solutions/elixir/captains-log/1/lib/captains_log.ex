defmodule CaptainsLog do
  @planetary_classes ["D", "H", "J", "K", "L", "M", "N", "R", "T", "Y"]

  @spec random_planet_class() :: String.t()
  def random_planet_class(), do: Enum.random(@planetary_classes)

  @spec random_ship_registry_number() :: String.t()
  def random_ship_registry_number(), do: "NCC-#{Enum.random(1000..9999)}"

  @spec random_stardate() :: number()
  def random_stardate(), do: :rand.uniform() * 1000 + 41000

  @spec format_stardate(float()) :: String.t()
  def format_stardate(stardate) do
    :io_lib.format("~.1f", [stardate]) |> to_string()
  end
end
