#!/bin/bash
S=/tmp/claude-0/-home-user-cad/6c4a94e9-9693-5ed3-ad36-1415c802a115/scratchpad
cd /home/user/cad
for m in m1 m2 m3 m1,m2,m3; do
  git checkout -q -- crates/topo/src
  python3 $S/mutant.py $m >> /home/user/mutants.done 2>&1
  export CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=3 CARGO_TARGET_DIR=/home/user/tgt-head CAD_TOLERANCE_EPS=1e-9
  cargo nextest run --profile ci --ignore-default-filter -p topo --lib --no-fail-fast -E 'test(/wall_root_rows|arc_ball_rows|circle_sphere_rows::a_line_through/)' > /home/user/mut-$m-topo.log 2>&1
  echo "$m topo $?" >> /home/user/mutants.done
  cargo nextest run --profile ci --ignore-default-filter -p sweep --test all --success-output immediate --no-fail-fast -E 'test(/^an_aimed_ray_from_far_reads_a_split_side_right::/)' > /home/user/mut-$m-sweep.log 2>&1
  echo "$m sweep $?" >> /home/user/mutants.done
done
git checkout -q -- crates/topo/src
echo ALLDONE >> /home/user/mutants.done
