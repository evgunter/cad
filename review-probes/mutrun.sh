#!/bin/bash
# usage: mutrun.sh m1 m2 ...  (sequential; restores tree after each)
cd /home/user/cad
for m in "$@"; do
  python3 /home/user/review-target/mut/mutants.py $m > /home/user/review-target/mut-$m.log 2>&1 || { echo "EXIT apply-failed" >> /home/user/review-target/mut-$m.log; continue; }
  date >> /home/user/review-target/mut-$m.log
  source /home/user/review-target/env.sh
  cargo nextest run -p topo -p sweep -p geom-brep --features sweep/test-support --no-fail-fast -E "$FILTER | test(/a_seamless_pipe_cut/)" >> /home/user/review-target/mut-$m.log 2>&1
  echo "EXIT $?" >> /home/user/review-target/mut-$m.log
  date >> /home/user/review-target/mut-$m.log
  git checkout -q -- crates/topo/src crates/geom-brep/src
done
echo ALLDONE > /home/user/review-target/mut-done.flag
