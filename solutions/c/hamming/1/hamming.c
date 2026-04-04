#include "hamming.h"

int compute(const char *lhs, const char *rhs)
{
  int acc = 0;

  while (*lhs) {
    if (!*rhs) {
      return -1;
    }

    if (*lhs != *rhs) {
      acc++;
    }

    lhs++;
    rhs++;
  }

  return *rhs ? -1 : acc;
}
