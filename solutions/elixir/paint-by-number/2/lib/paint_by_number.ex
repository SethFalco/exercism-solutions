defmodule PaintByNumber do
  @doc """
  This could be done with just:
  Integer.digits(color_count - 1, 2) |> length

  But the task calls for recursion and the power operator, so RIP. 🪦

  Normally I prefer to bitshift over using power as it's more performant. It
  should be possible to do Bitwise.bsr(color_count, 1) to step one bit at at
  time.
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
