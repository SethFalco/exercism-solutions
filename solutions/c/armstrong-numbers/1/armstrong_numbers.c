#include <math.h>
#include "armstrong_numbers.h"

bool is_armstrong_number(int candidate)
{
  double log = log10(candidate);
  int digits = (int)log == log ? log : log + 1;
  int remainder = candidate;

  for (int i = 0; i < digits; i++) {
    int digit = remainder % 10;
    remainder /= 10;
    candidate -= pow(digit, digits);
  }

  return candidate == 0;
}
