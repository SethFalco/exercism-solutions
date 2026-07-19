inputs = %{
  "1" => 1,
  "3" => 3,
  "64" => 64,
  "2147483647" => 2147483647,
}

defmodule PaintByNumber do
  def recursive_pow(x, y), do: (if 2 ** y < x, do: recursive_pow(x, y + 1), else: y)

  def recursive_bitshift(x, y) do
    z = Bitwise.bsr(x, 1)
    if z !== 0, do: recursive_bitshift(z, y + 1), else: y
  end

  def recursive_root(x, y), do: (if x ** (1 / y) >= 2, do: recursive_root(x, y + 1), else: y)

  def recursive_divide(x, y) do
    z = x / 2
    if z > 1, do: recursive_divide(z, y + 1), else: y
  end
end


Benchee.run(
  %{
    "Integer.digits(x - 1, 2) |> length" => &(Integer.digits(&1 - 1, 2) |> length),
    "recursive_pow" => &(PaintByNumber.recursive_pow(&1, 1)),
    "recursive_bitshift" => &(PaintByNumber.recursive_bitshift(&1 - 1, 1)),
    "recursive_root" => &(PaintByNumber.recursive_root(&1 - 1, 1)),
    "recursive_divide" => &(PaintByNumber.recursive_divide(&1, 1))
  },
  inputs: inputs,
  memory_time: 2
)
