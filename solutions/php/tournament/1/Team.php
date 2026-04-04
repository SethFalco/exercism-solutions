<?php

namespace SethFalco;

class Team
{
    private string $name;
    private int $matchesPlayed;
    private int $wins;
    private int $draws;
    private int $losses;

    public function __construct(string $name)
    {
        $this->name = $name;
        $this->matchesPlayed = 0;
        $this->wins = 0;
        $this->draws = 0;
        $this->losses = 0;
    }

    public function incMatchesPlayed(int $matchesPlayed = 1): int
    {
        return $this->matchesPlayed += $matchesPlayed;
    }

    public function getPoints(): int
    {
        return ($this->wins * 3) + ($this->draws);
    }

    public function incWins(int $wins = 1): int
    {
        return $this->wins += $wins;
    }

    public function incDraws(int $draws = 1): int
    {
        return $this->draws += $draws;
    }

    public function incLosses(int $losses = 1): int
    {
        return $this->losses += $losses;
    }

    public function getName(): string
    {
        return $this->name;
    }

    public function getMatchesPlayed(): int
    {
        return $this->matchesPlayed;
    }

    public function getWins(): int
    {
        return $this->wins;
    }

    public function getDraws(): int
    {
        return $this->draws;
    }

    public function getLosses(): int
    {
        return $this->losses;
    }
}
