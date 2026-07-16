defmodule RPG do
  defmodule Character do
    defstruct health: 100, mana: 0

    @type t :: %Character{health: non_neg_integer(), mana: non_neg_integer()}
  end

  defmodule LoafOfBread do
    defstruct []

    @type t :: %LoafOfBread{}
  end

  defmodule ManaPotion do
    defstruct strength: 10

    @type t :: %ManaPotion{strength: non_neg_integer()}
  end

  defmodule Poison do
    defstruct []

    @type t :: %Poison{}
  end

  defmodule EmptyBottle do
    defstruct []

    @type t :: %EmptyBottle{}
  end

  defprotocol Edible do
    @spec eat(any(), Character.t()) :: {any(), Character.t()}
    def eat(item, character)
  end

  defimpl Edible, for: LoafOfBread do
    def eat(_, character) do
      {nil, Map.put(character, :health, character.health + 5)}
    end
  end

  defimpl Edible, for: ManaPotion do
    @spec eat(ManaPotion.t(), Character.t()) :: {EmptyBottle.t(), Character.t()}
    def eat(item, character) do
      {%EmptyBottle{}, Map.put(character, :mana, character.mana + item.strength)}
    end
  end

  defimpl Edible, for: Poison do
    @spec eat(Poison.t(), Character.t()) :: {EmptyBottle.t(), Character.t()}
    def eat(_, character) do
      {%EmptyBottle{}, Map.put(character, :health, 0)}
    end
  end
end
