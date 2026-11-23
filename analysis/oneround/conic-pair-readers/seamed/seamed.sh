#!/bin/bash
A=/home/user/cad/analysis/oneround/conic-pair-readers
for t in head base; do for e in 1e-9 1e-12; do
  d=/home/user/runs/$t-$e
  ( CONIC_LADDER_OUT=$d CAD_TOLERANCE_EPS=$e /home/user/tgt-$t/debug/examples/ladder_seamed > $d/seamed.log 2>&1; python3 $A/oracle.py $d/ladder-seamed-eps$e.jsonl > $d/seamed-table.txt 2>&1; echo "exit $?" >> $d/seamed-table.txt ) &
done; done
wait; echo done > /home/user/runs/seamed.done
