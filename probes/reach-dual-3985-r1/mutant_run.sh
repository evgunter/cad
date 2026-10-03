#!/bin/bash
# usage: mut.sh NAME  (source already mutated in wt-mut)
S=/tmp/claude-0/-home-user-cad/72847030-e129-5e45-8b63-b29e7aa9fbdd/scratchpad
cd /home/user/wt-mut
git diff --stat crates/topo/src
FILT='test(/tilted_sphere_pair|wedge_through|join1_r1_rows|snowman|conic_edge_curved_face|germ_coplanar_conic|m5_pr5_tilted_cut|m5_pr9c|m5_s13_review|verbs_ga_r2|zz_probe|chord_join|splitting|r2_bool_door|poleguard|refusal_concision/)'
CARGO_INCREMENTAL=0 PROBE_OUT=$S/out/mut-$1 cargo nextest run -p topo -p sweep -p mesh -p editor-core -p step-import -E "$FILT" --no-fail-fast > $S/mut-$1.log 2>&1
grep -E '^\s+(FAIL|SIGABRT|TIMEOUT)' $S/mut-$1.log | sed 's/^ *//' | cut -c1-150
grep Summary $S/mut-$1.log
grep -h TALLY $S/out/mut-$1.* 2>/dev/null
