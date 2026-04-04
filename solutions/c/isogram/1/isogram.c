#include <ctype.h>
#include <stdio.h>
#include "isogram.h"

bool is_isogram(char* phrase) {
  if (!phrase) {
    return false;
  }

  char* start = phrase;

  for (int i = 'a'; i <= 'z'; i++) {
    bool found = false;

    while (*phrase) {
      if (i == tolower(*phrase)) {
        if (found) {
          return false;
        }

        found = true;
      }

      phrase++;
    }

    phrase = start;
  }

  return true;
}
