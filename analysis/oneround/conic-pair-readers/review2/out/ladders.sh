#!/bin/bash
A=/home/user/cad-head/analysis/oneround/conic-pair-readers
run() { t=$1; e=$2; d=/home/user/r2/$t-$e; mkdir -p $d
  ( cd $d; X=/home/user/tgt2-$t/debug/examples
    CONIC_LADDER_OUT=$d CAD_TOLERANCE_EPS=$e $X/ladder > ladder.log 2>&1; echo "ladder exit $?" >> run.log
    python3 $A/oracle.py $d/ladder-eps$e.jsonl > ladder-table.txt 2>&1; echo "oracle exit $?" >> run.log
    CONIC_LADDER_OUT=$d CAD_TOLERANCE_EPS=$e $X/ladder_wall > wall.log 2>&1; echo "wall exit $?" >> run.log
    python3 $A/oracle.py $d/ladder-wall-eps$e.jsonl > wall-table.txt 2>&1; echo "wall oracle exit $?" >> run.log
    CAD_TOLERANCE_EPS=$e $X/suspects_t1 > t1-$e.jsonl 2> t1.err; echo "t1 exit $?" >> run.log
    python3 $A/suspects/oracle_t1.py t1-$e.jsonl > t1-oracle.txt 2>&1; echo "t1 oracle exit $?" >> run.log ) &
}
run head 1e-9; run head 1e-12; run base 1e-9; run base 1e-12
wait; echo done > /home/user/r2/ladders.done
