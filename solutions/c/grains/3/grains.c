#include "grains.h"

uint64_t square(const uint8_t index)
{
  if (index == 0 || index > 64) {
    return 0;
  }

  return 1ul << (index - 1);
}

uint64_t total(void)
{
  uint64_t acc = 0;

  for (uint8_t i = 1; i <= 64; i++) {
    acc += square(i);
  }

  return acc;
}
