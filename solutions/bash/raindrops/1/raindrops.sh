#!/usr/bin/env bash

main () {
  if [ $# -ne 1 ]; then
    echo 'Incorrect number of parameters.'
    return 1
  fi

  num=${1}
  result=""

  if [ $((${num} % 3)) -eq 0 ]; then
    result="Pling"
  fi

  if [ $((${num} % 5)) -eq 0 ]; then
    result="${result}Plang"
  fi

  if [ $((${num} % 7)) -eq 0 ]; then
    result="${result}Plong"
  fi

  echo ${result:-${num}}
}

main "$@"