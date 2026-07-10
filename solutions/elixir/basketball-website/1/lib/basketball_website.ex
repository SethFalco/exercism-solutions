defmodule BasketballWebsite do
  @spec extract_from_path(Map, String.t()) :: String.t() | nil
  def extract_from_path(data, path) do
    do_extract_from_path(data, String.split(path, "."))
  end

  @spec do_extract_from_path(Map, [String.t()]) :: String.t() | nil
  defp do_extract_from_path(data, []), do: data

  defp do_extract_from_path(data, [head | tail]) do
    do_extract_from_path(data[head], tail)
  end

  @spec get_in_path(Map, String.t()) :: String.t() | nil
  def get_in_path(data, path) do
    Kernel.get_in(data, String.split(path, "."))
  end
end
