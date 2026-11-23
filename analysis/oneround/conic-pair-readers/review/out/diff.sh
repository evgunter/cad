#!/bin/bash
# the differential: head then base, eps 1e-9 then 1e-12
for t in head base; do
  w=/home/user/cad; [ $t = base ] && w=/home/user/cad-base
  cd $w
  for e in 1e-9 1e-12; do
    CAD_TOLERANCE_EPS=$e CARGO_TARGET_DIR=/home/user/tgt-$t cargo nextest run --profile ci -p geom -p geom-brep -p topo -p sweep -p step-import --status-level fail --final-status-level fail > /home/user/runs/diff-$t-$e.log 2>&1
    echo "exit $?" >> /home/user/runs/diff-$t-$e.log
  done
done
echo done > /home/user/runs/diff.done
