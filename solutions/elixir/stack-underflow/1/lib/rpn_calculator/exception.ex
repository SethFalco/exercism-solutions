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

  @spec divide([number()]) :: float()
  def divide(stack) do
    y = Enum.at(stack, 0)
    x = Enum.at(stack, 1)

    cond do
      x === nil or y === nil -> raise StackUnderflowError, "when dividing"
      y === 0 -> raise DivisionByZeroError
      true -> x / y
    end
  end
end
