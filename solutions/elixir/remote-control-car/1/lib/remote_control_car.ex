defmodule RemoteControlCar do
  @enforce_keys [:nickname]
  defstruct [
    :nickname,
    battery_percentage: 100,
    distance_driven_in_meters: 0
  ]

  @spec new() :: %RemoteControlCar{}
  def new() do
    %RemoteControlCar{
      nickname: "none",
      battery_percentage: 100,
      distance_driven_in_meters: 0
    }
  end

  @spec new(String.t()) :: %RemoteControlCar{}
  def new(nickname) do
    %RemoteControlCar{
      nickname: nickname,
      battery_percentage: 100,
      distance_driven_in_meters: 0
    }
  end

  @spec display_distance(%RemoteControlCar{}) :: String.t()
  def display_distance(remote_car) when is_struct(remote_car, RemoteControlCar) do
    "#{remote_car.distance_driven_in_meters} meters"
  end

  @spec display_battery(%RemoteControlCar{}) :: String.t()
  def display_battery(remote_car) when is_struct(remote_car, RemoteControlCar) do
    case remote_car.battery_percentage do
      0 -> "Battery empty"
      battery_percentage -> "Battery at #{battery_percentage}%"
    end
  end

  @spec drive(%RemoteControlCar{}) :: %RemoteControlCar{}
  def drive(remote_car) when is_struct(remote_car, RemoteControlCar) do
    if remote_car.battery_percentage === 0 do
      remote_car
    else
      %RemoteControlCar{
        nickname: remote_car.nickname,
        battery_percentage: remote_car.battery_percentage - 1,
        distance_driven_in_meters: remote_car.distance_driven_in_meters + 20
      }
    end
  end
end
