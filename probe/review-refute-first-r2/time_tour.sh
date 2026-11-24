#!/bin/bash
D=~/.local/share/cad-work/review-refute-first
cd $D
for i in 1 2 3; do for t in base head; do
  s=$(date +%s.%N); ./demo-tour-$t certified > tour_${t}_$i.out 2>&1; e=$(date +%s.%N)
  echo "$t $i $(echo "$e - $s" | bc)" >> times.log
done; done
echo DONE >> times.log
