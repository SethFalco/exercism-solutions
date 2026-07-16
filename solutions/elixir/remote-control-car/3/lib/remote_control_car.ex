defmodule RemoteControlCar do
  @enforce_keys [:nickname]
  defstruct [
    :nickname,
    battery_percentage: 100,
    distance_driven_in_meters: 0
  ]

  @type t :: %RemoteControlCar{
          nickname: String.t(),
          battery_percentage: non_neg_integer(),
          distance_driven_in_meters: non_neg_integer()
        }

  @spec new(String.t()) :: RemoteControlCar.t()
  def new(nickname \\ "none") do
    %RemoteControlCar{
      nickname: nickname,
      battery_percentage: 100,
      distance_driven_in_meters: 0
    }
  end

  @spec display_distance(RemoteControlCar.t()) :: String.t()
  def display_distance(remote_car) when is_struct(remote_car, RemoteControlCar) do
    "#{remote_car.distance_driven_in_meters} meters"
  end

  @spec display_battery(RemoteControlCar.t()) :: String.t()
  def display_battery(remote_car) when is_struct(remote_car, RemoteControlCar) do
    case remote_car.battery_percentage do
      0 -> "Battery empty"
      battery_percentage -> "Battery at #{battery_percentage}%"
    end
  end

  @spec drive(RemoteControlCar.t()) :: RemoteControlCar.t()
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
