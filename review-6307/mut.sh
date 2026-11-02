#!/bin/bash
# usage: mut.sh NAME  (applies /home/user/mut-NAME.py to cad-mut, runs rows, reverts)
set -u
N=$1
cd /home/user/cad-mut
git checkout -q -- crates/
python3 /home/user/mut-$N.py || { echo "APPLY FAIL" > /home/user/mut-$N.log; exit 1; }
git diff --stat >> /home/user/mut-$N.log
export CARGO_TARGET_DIR=/home/user/tgt-mut
REVIEW_FAST=1 cargo nextest run -p topo --lib --no-fail-fast -E 'test(/sphere_region|circle_roots|chart_singularity|review_every/)' >> /home/user/mut-$N.log 2>&1
cargo nextest run -p topo --no-fail-fast -E 'test(/props_sphere_cap_door|cone_apex_cap_body/)' >> /home/user/mut-$N.log 2>&1
for e in 1e-6 1e-9; do
CAD_TOLERANCE_EPS=$e cargo nextest run -p sweep --no-fail-fast -E 'test(/blend4_r1_probes::p2_slim_skews|one_region_rows::|a_tube_ending_on_a_ball::the_witness|band_planar_cut_off_interval::the_box|fillet_h5_hostless_rim_interval::|m5_s13_pips_interval::certified|m6_surgery_interval::certified|tilted_sphere_pair|a_wrap_tip_sits/)' >> /home/user/mut-$N.log 2>&1
done
git checkout -q -- crates/
echo MUTDONE >> /home/user/mut-$N.log
