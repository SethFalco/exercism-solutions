defmodule DancingDots.Animation do
  @type dot :: DancingDots.Dot.t()
  @type opts :: keyword
  @type error :: any
  @type frame_number :: pos_integer

  @callback init(opts) :: {:ok, opts} | {:error, error}
  @callback handle_frame(dot, frame_number, opts) :: dot

  defmacro __using__(_) do
    quote do
      @behaviour DancingDots.Animation
      def init(opts), do: {:ok, opts}
      defoverridable init: 1
    end
  end
end

defmodule DancingDots.Flicker do
  use DancingDots.Animation

  @doc """
  I benchmarked a few solutions for finding if the number is divisible by 4. In
  most languages/runtimes modulo/division is very slow, so I prefer Bitwise
  tricks. For example, checking if the last 2 bits are 0.

  However, benchmarks show that rem is super fast!

  rem(x, 4) === 0                   37.25 M
  Bitwise.band(x, 0b11) === 0       35.24 M - 1.06x slower +1.53 ns
  <<x::2>> === <<0::1, 0::1>>       20.32 M - 1.83x slower +22.36 ns

  rem(x, 4) === 0                         0 B
  Bitwise.band(x, 0b11) === 0             0 B - 1.00x memory usage +0 B
  <<x::2>> === <<0::1, 0::1>>            24 B - ∞ x memory usage +24 B
  """
  @impl DancingDots.Animation
  def handle_frame(dot, frame_number, _) do
    if rem(frame_number, 4) === 0 do
      Map.put(dot, :opacity, dot.opacity / 2)
    else
      dot
    end
  end
end

defmodule DancingDots.Zoom do
  use DancingDots.Animation

  @impl DancingDots.Animation
  def init(opts) do
    velocity = Keyword.get(opts, :velocity)

    if is_number(velocity) do
      {:ok, opts}
    else
      {:error,
       "The :velocity option is required, and its value must be a number. Got: #{inspect(velocity)}"}
    end
  end

  @impl DancingDots.Animation
  def handle_frame(dot, frame_number, velocity: velocity) do
    Map.put(dot, :radius, dot.radius + (frame_number - 1) * velocity)
  end
end
