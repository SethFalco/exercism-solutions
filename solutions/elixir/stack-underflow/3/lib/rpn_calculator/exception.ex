defmodule RPNCalculator.Exception do
  defmodule DivisionByZeroError do
    defexception message: "division by zero occurred"
  end

  defmodule StackUnderflowError do
    defexception message: "stack underflow occurred"

    @impl true
    def exception(value) do
      case value do
        [] ->
          %StackUnderflowError{}

        _ ->
          %StackUnderflowError{
            message: "stack underflow occurred, context: #{value}"
          }
      end
    end
  end

  @doc """
  My solution is not as performant as others in theory, but it actually takes
  from the end of the stack instead of the beginning.

  For these tests it doesn't matter as the test cases are limited to <= 2
  operands per test scenario. In practice if the numbers are reversed and
  meant to represent a stack, then we should be popping from the stack,
  especially if the stack has more than 2 operands left.
  """
  @spec divide([number()]) :: float()
  def divide(stack) do
    {x, stack} = List.pop_at(stack, -1)
    {y, _} = List.pop_at(stack, -1)

    cond do
      x === nil or y === nil -> raise StackUnderflowError, "when dividing"
      y === 0 -> raise DivisionByZeroError
      true -> x / y
    end
  end
end
