defmodule NameBadge do
  @spec print(integer(), String.t(), String.t()) :: String.t()
  def print(id, name, department) do
    department = if department, do: String.upcase(department), else: "OWNER"

    if id do
      "[#{id}] - #{name} - #{department}"
    else
      "#{name} - #{department}"
    end
  end
end
