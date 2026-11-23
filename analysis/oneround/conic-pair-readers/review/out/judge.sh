#!/bin/bash
A=/home/user/cad/analysis/oneround/conic-pair-readers
for t in head base; do for e in 1e-9 1e-12; do
  d=/home/user/runs/$t-$e
  ( python3 $A/oracle.py $d/ladder-eps$e.jsonl > $d/ladder-table.txt 2>&1; echo "oracle exit $?" >> $d/ladder-table.txt ) &
  ( cd $d && python3 $A/suspects/oracle_t1.py t1-$e.jsonl > t1-oracle.txt 2>&1; echo "oracle exit $?" >> t1-oracle.txt ) &
done; done
wait; echo done > /home/user/runs/judge.done
