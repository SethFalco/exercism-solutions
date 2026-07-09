defmodule LibraryFees do
  @spec datetime_from_string(String.t()) :: NaiveDateTime.t()
  def datetime_from_string(string), do: NaiveDateTime.from_iso8601!(string)

  @spec before_noon?(NaiveDateTime.t()) :: boolean()
  def before_noon?(datetime) do
    time = NaiveDateTime.to_time(datetime)
    Time.compare(time, ~T[12:00:00]) === :lt
  end

  @spec return_date(NaiveDateTime.t()) :: Date.t()
  def return_date(checkout_datetime) do
    days = if before_noon?(checkout_datetime), do: 28, else: 29
    due_date = NaiveDateTime.add(checkout_datetime, days, :day)
    NaiveDateTime.to_date(due_date)
  end

  @spec days_late(Date.t(), NaiveDateTime.t()) :: integer()
  def days_late(planned_return_date, actual_return_datetime) do
    actual_return_date = NaiveDateTime.to_date(actual_return_datetime)

    if Date.compare(actual_return_date, planned_return_date) === :lt do
      0
    else
      Date.diff(actual_return_date, planned_return_date)
    end
  end

  @spec monday?(NaiveDateTime.t()) :: boolean()
  def monday?(datetime), do: Date.day_of_week(datetime) === 1

  @spec calculate_late_fee(String.t(), String.t(), integer()) :: integer()
  def calculate_late_fee(checkout, return, rate) do
    checkout_datetime = datetime_from_string(checkout)
    return_datetime = datetime_from_string(return)
    due_date = return_date(checkout_datetime)
    fee = days_late(due_date, return_datetime) * rate
    fee = if monday?(return_datetime), do: fee * 0.5, else: fee
    floor(fee)
  end
end
