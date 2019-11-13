#!/bin/bash

DAY=$1
if [ -z "$DAY" ]; then
  echo "No day supplied"
  exit 1
fi

INPUT="${@:2}"

if [ -z "$INPUT" ]; then
  INPUT="../input/day$DAY"
fi


mkdir .build &> /dev/null

set -e

kotlinc-native day$DAY.kt helpers.kt -o .build/day$DAY

.build/day$DAY.kexe < "$INPUT"
