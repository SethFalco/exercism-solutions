#!/usr/bin/env bash

sound_check () {
  factor=${1}
  sound=${2}

  if [ $((${num} % ${factor})) -eq 0 ]; then
    result="${result}${sound}"
  fi
}

main () {
  if [ $# -ne 1 ]; then
    echo 'Incorrect number of parameters.'
    return 1
  fi

  num=${1}

  sound_check 3 Pling
  sound_check 5 Plang
  sound_check 7 Plong

  echo ${result:-${num}}
}

main "$@"