#!/bin/bash
for t in head base; do
  cd /home/user/cad-$t
  for e in 1e-9 1e-12; do
    CAD_TOLERANCE_EPS=$e CARGO_TARGET_DIR=/home/user/tgt2-$t cargo nextest run --profile ci -p geom -p geom-brep -p topo -p sweep -p step-import --status-level fail --final-status-level fail > /home/user/r2/diff-$t-$e.log 2>&1
    echo "exit $?" >> /home/user/r2/diff-$t-$e.log
  done
done
echo done > /home/user/r2/diff.done
