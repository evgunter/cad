#!/bin/bash
# the survivors against the slow set's oracles for curve_pair (window oracle, meet oracle), which the per-PR run selects when the diff edits their files
W=/home/user/cad-mut; M=/home/user/cad/analysis/oneround/conic-pair-readers/review/mutants; O=/home/user/runs/mut2
cd $W
run() {
  for e in 1e-9 1e-12; do
    CAD_TOLERANCE_EPS=$e CARGO_TARGET_DIR=/home/user/tgt-mut cargo nextest run --profile ci --ignore-default-filter -p geom-brep \
      -E 'test(/window::oracle|meet_oracle|one_curve_rows|touch_spread/)' --status-level fail --final-status-level fail --no-fail-fast > $O/$1-$e.log 2>&1
    echo "exit $?" >> $O/$1-$e.log
  done
}
git checkout -q -- . ; run BASELINE
for n in R4-level-jet-third-sign R9-window-height-fourth-zero S2-level-ddp-from-centre R1-over-level-dropped R3-mean-arc-is-the-anchors-height S1-foot-seed-from-centre; do
  git checkout -q -- . && git apply $M/$n.patch && run $n
  git checkout -q -- .
done
echo done > /home/user/runs/mut2.done
