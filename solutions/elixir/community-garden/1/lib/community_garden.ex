defmodule Plot do
  @enforce_keys [:plot_id, :registered_to]
  defstruct [:plot_id, :registered_to]
end

defmodule CommunityGarden do
  def start(opts \\ []) do
    Agent.start(fn -> {[], 1} end)
  end

  def list_registrations(pid) do
    Agent.get(pid, fn {plots, _} -> plots end)
  end

  def register(pid, register_to) do
    Agent.get_and_update(pid, fn {plots, next_id} ->
      plot = %Plot{
        plot_id: next_id,
        registered_to: register_to
      }

      {plot, {[plot | plots], next_id + 1}}
    end)
  end

  def release(pid, plot_id) do
    Agent.update(pid, fn {plots, next_id} ->
      {Enum.filter(plots, fn p -> p.plot_id !== plot_id end), next_id}
    end)
  end

  def get_registration(pid, plot_id) do
    Agent.get(pid, fn {plots, _} ->
      plot = Enum.find(plots, fn p -> p.plot_id === plot_id end)

      if plot do
        plot
      else
        {:not_found, "plot is unregistered"}
      end
    end)
  end
end
