<?php

declare(strict_types=1);

const GIGASECOND = 'PT1000000000S';

function from(DateTimeImmutable $date): DateTimeImmutable
{
    return $date->add(new DateInterval(GIGASECOND));
}
