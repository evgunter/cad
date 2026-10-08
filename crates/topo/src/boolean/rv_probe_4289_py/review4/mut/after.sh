#!/bin/bash
R=/home/user/probe/crates/topo/src/boolean/rv_probe_4289_py/review4; O=$R/mut/results
until grep -q MUTDONE $O/summary.txt; do sleep 20; done
cd /home/user/probe; export CARGO_TARGET_DIR=/home/user/tgt2
# BndLever on the graze families at 1e-6
git checkout -q crates/topo/src/boolean/sectors.rs; python3 $R/mut/mut.py BndLever
cargo test -p topo --lib --no-run > $O/BndLever.build 2>&1
BIN=$(ls -t /home/user/tgt2/debug/deps/topo-* | grep -v '\.d$' | head -1)
$BIN boolean::sectors boolean::vtxfac > $O/BndLever.unit 2>&1
cd $R; for s in 1 2; do RV_CASES=cases_g$s.txt RV_OUT=mut/results/out_BndLever_g${s}_6.txt RV_EPS=1e-6 $BIN --exact boolean::sectors::rv_probe_4289::rv_probe_cases --test-threads 1 >/dev/null 2>&1; done
echo "BndLever: unit fails: $(grep -E '^test .* FAILED$' $O/BndLever.unit | tr '\n' ' ')" >> $O/summary.txt
# ErrOffFace against the germ oracle and the two-pass matrix
cd /home/user/probe; git checkout -q crates/topo/src/boolean/sectors.rs; python3 $R/mut/mut.py ErrOffFace
cargo nextest run -p topo --no-fail-fast -E 'test(=every_edge_a_vertex_read_again_reads_is_classed_against_the_germ) | test(=every_scene_builds_sound_or_refuses_typed_at_every_pose) | binary(a_vertex_read_by_two_sector_passes)' > $O/ErrOffFace.integ 2>&1
echo "ErrOffFace integ: $(grep -E '^\s+Summary|FAIL \[' $O/ErrOffFace.integ | tr '\n' ' ')" >> $O/summary.txt
git checkout -q crates/topo/src/boolean/sectors.rs
echo AFTERDONE >> $O/summary.txt
