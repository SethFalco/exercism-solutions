#include <math.h>
#include "armstrong_numbers.h"

bool is_armstrong_number(int candidate)
{
  int digits = log10(candidate) + 1;
  int remainder = candidate;

  while (remainder) {
    int digit = remainder % 10;
    remainder /= 10;
    candidate -= pow(digit, digits);
  }

  return candidate == 0;
}
