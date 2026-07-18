defmodule TopSecret do
  @spec to_ast(String.t()) :: Macro.t()
  def to_ast(string), do: Code.string_to_quoted!(string)

  @spec decode_secret_message_part(Macro.t(), [String.t()]) :: {Macro.t(), [String.t()]}
  def decode_secret_message_part({op, _, [{def_name, _, def_args} | _]} = ast, acc)
      when op === :def or op === :defp do
    {def_name, def_args} =
      cond do
        def_name === :when ->
          [{def_name, _, def_args} | _] = def_args
          {def_name, def_args}

        true ->
          {def_name, def_args}
      end

    length = if is_list(def_args), do: length(def_args), else: 0
    msg = to_string(def_name) |> String.slice(0, length)
    {ast, [msg | acc]}
  end

  def decode_secret_message_part(ast, acc), do: {ast, acc}

  @spec decode_secret_message(String.t()) :: String.t()
  def decode_secret_message(string) do
    {_, msg} = Macro.prewalk(to_ast(string), [], &decode_secret_message_part/2)
    Enum.reverse(msg) |> Enum.join()
  end
end
