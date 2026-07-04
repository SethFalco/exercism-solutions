defmodule HighScore do
  @default_initial_score 0

  def new(), do: %{}

  def add_player(scores, name, score \\ @default_initial_score) do
    Map.put(scores, name, score)
  end

  def remove_player(scores, name), do: Map.delete(scores, name)

  def reset_score(scores, name) do
    if name in scores do
      Map.replace(scores, name, @default_initial_score)
    else
      add_player(scores, name, @default_initial_score)
    end
  end

  def update_score(scores, name, score) do
    prev_score = Map.get(scores, name)

    if prev_score !== nil do
      Map.replace(scores, name, prev_score + score)
    else
      add_player(scores, name, score)
    end
  end

  def get_players(scores), do: Map.keys(scores)
end
