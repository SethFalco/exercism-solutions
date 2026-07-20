defmodule PaintByNumber do
  @doc """
  This could be done with just: Integer.digits(x - 1, 2) |> length

  Though benchmarks show that's both slower and uses more memory.

  I usually prefer bitshift operations for performance, and can confirm this
  task is possible using Bitwise.bsr/2. See my benchmark in benchmark.exs!

  I use neither of those here though as the task specifically calls for
  recursion and the power operator. Benchmark results:

  recursive_bitshift                       31.12 M
  Integer.digits(x - 1, 2) |> length       12.87 M - 2.42x slower +45.55 ns
  recursive_pow                            12.22 M - 2.55x slower +49.69 ns
  recursive_divide                         10.19 M - 3.06x slower +66.05 ns
  recursive_root                            3.80 M - 8.20x slower +231.37 ns

  recursive_bitshift                             0 B
  Integer.digits(x - 1, 2) |> length            96 B - ∞ x memory usage +96 B
  recursive_pow                                  0 B - 1.00x memory usage +0 B
  recursive_divide                              96 B - ∞ x memory usage +96 B
  recursive_root                               192 B - ∞ x memory usage +192 B
  """
  def palette_bit_size(color_count), do: do_palette_bit_size(color_count, 1)

  defp do_palette_bit_size(color_count, count) do
    result = 2 ** count

    if result < color_count do
      do_palette_bit_size(color_count, count + 1)
    else
      count
    end
  end

  def empty_picture(), do: <<>>

  def test_picture(), do: <<0::2, 1::2, 2::2, 3::2>>

  def prepend_pixel(picture, color_count, pixel_color_index) do
    bit_size = palette_bit_size(color_count)
    <<pixel_color_index::size(bit_size), picture::bitstring>>
  end

  def get_first_pixel(<<>>, _), do: nil

  def get_first_pixel(picture, color_count) do
    bit_size = palette_bit_size(color_count)
    <<first::size(^bit_size), _::bitstring>> = <<picture::bitstring>>
    first
  end

  def drop_first_pixel(<<>>, _), do: <<>>

  def drop_first_pixel(picture, color_count) do
    bit_size = palette_bit_size(color_count)
    <<_::size(^bit_size), rest::bitstring>> = <<picture::bitstring>>
    rest
  end

  def concat_pictures(picture1, picture2) do
    <<picture1::bitstring, picture2::bitstring>>
  end
end
