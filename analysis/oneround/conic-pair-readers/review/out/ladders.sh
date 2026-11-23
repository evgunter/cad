#!/bin/bash
run() { # tree eps
  t=$1; e=$2; d=/home/user/runs/$t-$e; mkdir -p $d
  ( cd /home/user/runs; CONIC_LADDER_OUT=$d CAD_TOLERANCE_EPS=$e /home/user/tgt-$t/debug/examples/ladder > $d/ladder.log 2>&1; echo "ladder exit $?" >> $d/ladder.log
    CAD_TOLERANCE_EPS=$e /home/user/tgt-$t/debug/examples/suspects_t1 > $d/t1-$e.jsonl 2> $d/t1.err; echo "t1 exit $?" >> $d/ladder.log ) &
}
run head 1e-9; run head 1e-12; run base 1e-9; run base 1e-12
wait
echo ALLDONE > /home/user/runs/ladders.done
