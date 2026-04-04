<?php

declare(strict_types=1);

require_once('Team.php');

use SethFalco\Team;

class Tournament
{
    public const TABLE_FORMAT = '%-30s | %2s | %2s | %2s | %2s | %2s';

    public function tally($scores)
    {
        $results = [];
        $matches = explode("\n", $scores);

        foreach ($matches as $match) {
            if (strlen($match) === 0) {
                break;
            }

            list($teamOneName, $teamTwoName, $matchState) = explode(';', $match, 3);

            if (! array_key_exists($teamOneName, $results)) {
                $results[$teamOneName] = new Team($teamOneName);
            }

            if (! array_key_exists($teamTwoName, $results)) {
                $results[$teamTwoName] = new Team($teamTwoName);
            }

            $teamOne = $results[$teamOneName];
            $teamTwo = $results[$teamTwoName];

            $teamOne->incMatchesPlayed();
            $teamTwo->incMatchesPlayed();

            switch ($matchState) {
                case 'win':
                    $teamOne->incWins();
                    $teamTwo->incLosses();
                    break;
                case 'draw':
                    $teamOne->incDraws();
                    $teamTwo->incDraws();
                    break;
                case 'loss':
                    $teamOne->incLosses();
                    $teamTwo->incWins();
                    break;
                default:
                    throw new UnexpectedValueException("Received invalid match result: ${matchState}");
            }
        }

        uasort($results, function ($a, $b) {
            if ($a->getPoints() === $b->getPoints()) {
                return ord($a->getName()) - ord($b->getName());
            }

            return $b->getPoints() - $a->getPoints();
        });

        $array = [];
        $array[] = sprintf(Tournament::TABLE_FORMAT, 'Team', 'MP', 'W', 'D', 'L', 'P');

        foreach ($results as $result) {
            $array[] = sprintf(
                Tournament::TABLE_FORMAT,
                $result->getName(),
                $result->getMatchesPlayed(),
                $result->getWins(),
                $result->getDraws(),
                $result->getLosses(),
                $result->getPoints()
            );
        }

        return implode("\n", $array);
    }
}
