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

kotlinc day$DAY.kt -include-runtime -d .build/day$DAY.jar

java -jar .build/day$DAY.jar < "$INPUT"
