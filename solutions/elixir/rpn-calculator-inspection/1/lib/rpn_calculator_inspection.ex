defmodule RPNCalculatorInspection do
  @spec start_reliability_check(any(), String.t()) :: %{pid: pid(), input: String.t()}
  def start_reliability_check(calculator, input) do
    pid = spawn_link(fn -> calculator.(input) end)
    %{pid: pid, input: input}
  end

  @spec await_reliability_check_result(%{pid: pid(), input: String.t()}, map()) :: map()
  def await_reliability_check_result(%{pid: pid, input: input}, results) do
    status =
      receive do
        {:EXIT, ^pid, :normal} -> :ok
        {:EXIT, ^pid, _} -> :error
      after
        100 -> :timeout
      end

    Map.put(results, input, status)
  end

  @spec reliability_check(any(), [String.t()]) :: map()
  def reliability_check(calculator, inputs) do
    original = Process.flag(:trap_exit, true)
    tasks = Enum.map(inputs, &start_reliability_check(calculator, &1))
    result = Enum.reduce(tasks, %{}, &await_reliability_check_result(&1, &2))

    Process.flag(:trap_exit, original)
    result
  end

  @spec correctness_check(any(), [String.t()]) :: [number()]
  def correctness_check(calculator, inputs) do
    tasks = Enum.map(inputs, &Task.async(fn -> calculator.(&1) end))
    Enum.map(tasks, &Task.await(&1, 100))
  end
end
