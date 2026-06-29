defmodule GuessingGame do
  def compare(_) do
    "Make a guess"
  end

  def compare(secret_number, :no_guess) do
    compare(secret_number)
  end

  def compare(secret_number, guess) do
    cond do
      secret_number === guess -> "Correct"
      secret_number === guess + 1 || secret_number == guess - 1 -> "So close"
      secret_number < guess -> "Too high"
      secret_number > guess -> "Too low"
    end
  end
end
