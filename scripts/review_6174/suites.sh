#!/bin/bash
for tree in head base; do
  dir=/home/user/cad; [ $tree = base ] && dir=/home/user/base
  for eps in 1e-9 1e-6 1e-12; do
    cd $dir
    CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=3 CARGO_TARGET_DIR=/home/user/tgt-$tree CAD_TOLERANCE_EPS=$eps \
      cargo nextest run --profile ci --ignore-default-filter -p topo -p geom -p sweep -p editor-core --no-fail-fast \
      > /home/user/suite-$tree-$eps.log 2>&1
    echo "$tree $eps exit $?" >> /home/user/suites.done
  done
done
echo ALLDONE >> /home/user/suites.done
