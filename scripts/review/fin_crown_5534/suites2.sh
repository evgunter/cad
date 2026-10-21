#!/bin/bash
# One heavy job at a time: base and head, three eps rows each.
for eps in 1e-9 1e-6 1e-12; do
  for tree in head base; do
    cd /home/user/wt-$tree
    start=$(date +%s)
    CAD_TOLERANCE_EPS=$eps CARGO_TARGET_DIR=/home/user/tgt-$tree cargo nextest run --release --profile ci --ignore-default-filter -p topo -p sweep -p editor-core --no-fail-fast > /home/user/logs/suite2-$tree-$eps.log 2>&1
    echo "exit $? secs $(( $(date +%s) - start ))" >> /home/user/logs/suite2-$tree-$eps.log
    cp /home/user/tgt-$tree/nextest/ci/junit.xml /home/user/logs/junit2-$tree-$eps.xml 2>/dev/null
  done
done
echo ALLDONE > /home/user/logs/suites2.done
