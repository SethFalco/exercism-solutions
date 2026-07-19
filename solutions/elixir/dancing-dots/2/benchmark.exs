inputs = %{
  "0" => 0,
  "4" => 4,
  "9" => 9,
  "10" => 10,
  "64" => 64,
  "2147483647" => 2147483647,
}

Benchee.run(
  %{
    "<<x::2>> === <<0::1, 0::1>>" => fn x -> <<x::2>> === <<0::1, 0::1>> end,
    "rem(x, 4) === 0" => fn x -> rem(x, 4) === 0 end,
    "Bitwise.band(x, 0b11) === 0" => fn x -> Bitwise.band(x, 0b11) === 0 end
  },
  inputs: inputs,
  memory_time: 2
)
