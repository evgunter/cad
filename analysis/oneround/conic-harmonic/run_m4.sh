#!/bin/bash
# M4: the ci profile of geom-brep and topo, as the branch stands (mode 0)
# and with every conic read through the scratch polynomial (mode 6).
cd /home/user/cad
export CARGO_TARGET_DIR=/home/user/conic-probe-target
OUT=/home/user/conic-probe-out/m4
for eps in 1e-9 1e-12; do
  for mode in 0 6; do
    tag=eps$eps-mode$mode
    mkdir -p $OUT/$tag
    CAD_FUZZ_SEED=0x00c0ffee1234abcd CAD_TOLERANCE_EPS=$eps CONIC_PROBE_MODE=$mode CONIC_PROBE_LOG=$OUT/$tag \
      cargo nextest run -p geom-brep -p topo --profile ci --no-fail-fast > $OUT/$tag.log 2>&1
    echo "exit $?" >> $OUT/$tag.log
    cp $CARGO_TARGET_DIR/nextest/ci/junit.xml $OUT/$tag.junit.xml 2>/dev/null
    echo "$tag done $(date +%T)" >> $OUT/progress
  done
done
echo ALL-DONE >> $OUT/progress
