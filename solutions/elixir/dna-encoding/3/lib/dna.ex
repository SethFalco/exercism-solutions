defmodule DNA do
  @spec encode_nucleotide(number()) :: number()
  def encode_nucleotide(code_point) do
    case code_point do
      ?\s -> 0b0000
      ?A -> 0b0001
      ?C -> 0b0010
      ?G -> 0b0100
      ?T -> 0b1000
    end
  end

  @spec encode_nucleotide(number()) :: number()
  def decode_nucleotide(encoded_code) do
    case encoded_code do
      0b0000 -> ?\s
      0b0001 -> ?A
      0b0010 -> ?C
      0b0100 -> ?G
      0b1000 -> ?T
    end
  end

  @spec encode(charlist()) :: bitstring()
  def encode(dna), do: do_encode(dna, <<>>)

  @spec do_encode(charlist(), bitstring()) :: bitstring()
  defp do_encode([], result), do: result

  defp do_encode([head | tail], result) do
    val = encode_nucleotide(head)
    do_encode(tail, <<result::bitstring, val::4>>)
  end

  @spec decode(bitstring()) :: charlist()
  def decode(dna), do: do_decode(dna, ~c"")

  @spec do_decode(bitstring(), charlist()) :: charlist()
  defp do_decode(<<>>, result), do: result

  defp do_decode(<<first::4, rest::bitstring>>, result) do
    val = decode_nucleotide(first)
    do_decode(rest, result ++ [val])
  end
end
