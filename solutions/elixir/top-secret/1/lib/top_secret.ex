defmodule TopSecret do
  @spec to_ast(String.t()) :: Macro.t()
  def to_ast(string) do
    {_, ast} = Code.string_to_quoted(string)
    ast
  end

  @spec decode_secret_message_part(Macro.t(), [String.t()]) :: {Macro.t(), [String.t()]}
  def decode_secret_message_part(ast, acc)
      when not is_tuple(ast) or elem(ast, 0) not in [:def, :defp],
      do: {ast, acc}

  def decode_secret_message_part(ast, acc) do
    {_, _, [{def_name, _, def_args} | _]} = ast

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

  @spec decode_secret_message(String.t()) :: String.t()
  def decode_secret_message(string) do
    {_, msg} = Macro.prewalk(to_ast(string), [], &decode_secret_message_part/2)
    Enum.reverse(msg) |> Enum.join()
  end
end
