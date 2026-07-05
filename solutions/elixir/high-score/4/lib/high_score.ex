defmodule HighScore do
  @default_initial_score 0

  @spec new() :: %{}
  def new(), do: %{}

  @spec add_player(map(), String.t(), integer()) :: map()
  def add_player(scores, name, score \\ @default_initial_score) do
    Map.put(scores, name, score)
  end

  @spec remove_player(map(), String.t()) :: map()
  def remove_player(scores, name), do: Map.delete(scores, name)

  @spec reset_score(map(), String.t()) :: map()
  def reset_score(scores, name) do
    if name in scores do
      Map.replace(scores, name, @default_initial_score)
    else
      add_player(scores, name, @default_initial_score)
    end
  end

  @spec update_score(map(), String.t(), number()) :: map()
  def update_score(scores, name, score) do
    Map.update(scores, name, score, fn (val) -> val + score end)
  end

  @spec get_players(map()) :: [String.t()]
  def get_players(scores), do: Map.keys(scores)
end
