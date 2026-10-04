#!/bin/bash
cd /home/user/d2
while ! grep -q "^exit" logs/suite-topo-head-1e-6.log 2>/dev/null; do sleep 10; done
for m in oldenc fanorigin exempt inflane; do
  for e in 1e-9 1e-12; do
    ./suite.sh mut $e "-p topo" topo $m
    cd /home/user/wt-mut
    DMUT=$m CAD_TOLERANCE_EPS=$e CARGO_TARGET_DIR=/home/user/t-mut timeout 7000 cargo nextest run -p sweep --test all --no-capture --no-fail-fast -E 'test(far_thin_disc) | test(a_box_less_a_far_thin_disc) | test(d2_disc_family) | test(d2_quadrature_kind) | test(d2_revolved_kinds) | test(d2_planar_fans) | test(dprobe_)' > /home/user/d2/logs/mutsweep-$m-$e.log 2>&1
    echo "exit $?" >> /home/user/d2/logs/mutsweep-$m-$e.log
    cd /home/user/d2
  done
done
echo ALLDONE > logs/mutants.done
