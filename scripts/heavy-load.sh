#!/bin/sh

i=0

for (( ;; )) do
  if [ $i -gt 1000 ]; then
    exit 0
  fi

  echo Adding $i...
  oi run "oh man good luck ${i}s 20h"

  i=$((i+1))
  sleep 0.0005
done
