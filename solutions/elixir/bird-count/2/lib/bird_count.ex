defmodule BirdCount do
  def today(list), do: list[0]

  def increment_day_count([]), do: [1]
  def increment_day_count(list) do
    List.replace_at(list, 0, today(list) + 1)
  end

  def has_day_without_birds?([]), do: false
  def has_day_without_birds?(list) do
    if today(list) === 0 do
      true
    else
      { _, tail } = List.pop_at(list, 0)
      has_day_without_birds?(tail)
    end
  end

  def total([]), do: 0
  def total([head | tail]), do: head + total(tail)

  def busy_days([]), do: 0
  def busy_days(list) do
    { head, tail } = List.pop_at(list, 0)

    if head >= 5 do
      1 + busy_days(tail)
    else
      busy_days(tail)
    end
  end
end
