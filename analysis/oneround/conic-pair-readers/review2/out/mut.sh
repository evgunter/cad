#!/bin/bash
W=/home/user/cad-mut; M=/home/user/cad/analysis/oneround/conic-pair-readers/review2/mutants; O=/home/user/r2/mut
cd $W
run() {
  for e in 1e-9 1e-12; do
    CAD_TOLERANCE_EPS=$e CARGO_TARGET_DIR=/home/user/tgt2-mut cargo nextest run --profile ci -p geom -p geom-brep \
      -E 'test(/door_rows|arc_held_rows|curve_pair::|curve_face::|conic|on_carrier/)' --status-level fail --final-status-level fail --no-fail-fast > $O/$1-$e.log 2>&1
    echo "exit $?" >> $O/$1-$e.log
  done
}
git checkout -q -- . ; run BASELINE
for p in $M/*.patch; do
  n=$(basename $p .patch)
  git checkout -q -- . && git apply $p && run $n
  git checkout -q -- .
done
echo done > /home/user/r2/mut.done
