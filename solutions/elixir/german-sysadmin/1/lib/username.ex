defmodule Username do
  @alphabet ~c"abcdefghijklmnopqrstuvwxyz_"

  @spec sanitize([integer()]) :: [integer()]
  def sanitize(username) do
    Enum.flat_map(username, fn char ->
      case char do
        ?ä -> ~c"ae"
        ?ö -> ~c"oe"
        ?ü -> ~c"ue"
        ?ß -> ~c"ss"
        _ -> if char in @alphabet do [char] else [] end
      end
    end)
  end
end
