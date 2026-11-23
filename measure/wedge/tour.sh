#!/usr/bin/env bash
set -u
cd /home/user/cad/demos/tour
export CARGO_TARGET_DIR=/home/user/wm-tour-target CARGO_INCREMENTAL=0
W=/home/user/wm-work
echo "TOUR build start $(date +%s)" >> $W/tour-progress
#cargo build --release --example embed_walk > $W/tour-build.out 2>&1 || { echo "TOUR BUILD FAILED" >> $W/tour-progress; exit 1; }
for RUN in P M; do
  rm -rf $W/tlog-$RUN; mkdir -p $W/tlog-$RUN
  echo "TOUR $RUN start $(date +%s)" >> $W/tour-progress
  if [ "$RUN" = M ]; then MUT="CAD_WM_MUTANT=1"; else MUT="CAD_WM_X=0"; fi
  env CAD_GATE_PASS=1 CAD_WM_LOG=$W/tlog-$RUN $MUT /home/user/wm-tour-target/release/examples/embed_walk > $W/tour-$RUN.txt 2> $W/tour-$RUN.err
  echo "TOUR $RUN done $(date +%s) $(grep 'WALK TOTAL' $W/tour-$RUN.txt)" >> $W/tour-progress
done
