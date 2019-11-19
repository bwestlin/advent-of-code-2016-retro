#!/bin/bash

DAY=$1
if [ -z "$DAY" ]; then
  echo "No day supplied"
  exit 1
fi

INPUT=""
if [ "$2" == "timeit" ]; then
  export TIMEIT=1
  INPUT="${@:3}"
else
  INPUT="${@:2}"
fi

if [ -z "$INPUT" ]; then
  INPUT="../input/day$DAY"
fi


mkdir .build &> /dev/null

set -e

kotlinc day$DAY.kt helpers.kt -include-runtime -d .build/day$DAY.jar

java -jar .build/day$DAY.jar "$INPUT"
