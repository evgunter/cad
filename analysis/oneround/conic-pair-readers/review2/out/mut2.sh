#!/bin/bash
W=/home/user/cad-mut; M=/home/user/cad/analysis/oneround/conic-pair-readers/review2/mutants; O=/home/user/r2/mut2
cd $W
git checkout -q -- . && git apply $M/Q1-every-line-conic-and-conic-conic-pose-refuses.patch
for e in 1e-6 1e-9 1e-12; do
  CAD_TOLERANCE_EPS=$e CARGO_TARGET_DIR=/home/user/tgt2-mut cargo nextest run --profile ci -p geom-brep -E 'test(/arc_held_rows/)' --status-level fail --final-status-level fail --no-fail-fast > $O/Q1-$e.log 2>&1; echo "exit $?" >> $O/Q1-$e.log
done
git checkout -q -- .
slow() { CAD_TOLERANCE_EPS=1e-9 CARGO_TARGET_DIR=/home/user/tgt2-mut cargo nextest run --profile ci --ignore-default-filter -p geom-brep -E 'test(/curve_pair::/)' --status-level fail --final-status-level fail --no-fail-fast > $O/slow-$1.log 2>&1; echo "exit $?" >> $O/slow-$1.log; }
slow BASELINE
for n in N5-level-fourth-drops-x0 N7-one-curve-gap-uncharged N8-slant-floor-turn-swapped S2-level-ddp-from-centre; do
  git checkout -q -- . && git apply $M/$n.patch && slow $n; git checkout -q -- .
done
echo done > /home/user/r2/mut2.done
