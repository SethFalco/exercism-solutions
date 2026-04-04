#include <string.h>
#include "grade_school.h"

void init_roster(roster_t* roster)
{
  roster->count = 0;
}

bool add_student(roster_t* roster, char* name, uint8_t grade)
{
  int insert_index = 0;
  for (size_t i = 0; i < roster->count; i++) {
    const int name_cmp = strcmp(name, roster->students[i].name);
    if (name_cmp == 0) {
      return false;
    }

    uint8_t iter_grade = roster->students[i].grade;
    if ((grade == iter_grade && name_cmp < 0) || grade < iter_grade) {
      break;
    }
    insert_index++;
  }

  for (int i = roster->count - 1; i >= insert_index; i--) {
    roster->students[i + 1] = roster->students[i];
  }

  roster->students[insert_index].grade = grade;
  strcpy(roster->students[insert_index].name, name);

  roster->count++;
  return true;
}

roster_t get_grade(roster_t* roster, uint8_t grade)
{
  roster_t new_roster = {0};

  for (size_t i = 0; i < roster->count; i++) {
    if (roster->students[i].grade == grade) {
      new_roster.students[new_roster.count] = roster->students[i];
      new_roster.count++;
    }
  }

  return new_roster;
}
