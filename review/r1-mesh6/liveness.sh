#!/bin/bash
# Liveness of the two guards END TO END, in the RELEASE profile as the manifest ships it,
# and under the future manifest (debug-assertions off). Applies mutants, restores from HEAD.
set -u; cd /root/lanes/mesh-6r1; export CARGO_TARGET_DIR=/root/lanes/mesh-6r1/target
C=crates/mesh/src/curved.rs; T=crates/mesh/src/tessellate.rs
restore(){ git show HEAD:$1 > $1; }
BIN() { ls -t target/release/deps/all-* | grep -v '\.d$' | head -1; }
echo "== L1: pole floor removed (pole_columns returns nu) — the identified-edge census should fire on the ball/cone in RELEASE"
restore $C; python3 - <<'PY'
p='crates/mesh/src/curved.rs'; s=open(p).read()
old="    if has_pole && nu == 2 { 3 } else { nu }"; assert s.count(old)==1
open(p,'w').write(s.replace(old,"    let _ = has_pole; nu"))
PY
local-scripts/with-build-slot.sh -- cargo test --release -p mesh --test all --no-run 2>&1 | tail -1
$(BIN) r2_byte_stability_report --test-threads=1 2>&1 | grep -E 'panicked|identified-vertex|test result' | head -5
echo "-- same mutant, release with CARGO_PROFILE_RELEASE_DEBUG_ASSERTIONS=false (the post-publish manifest):"
CARGO_PROFILE_RELEASE_DEBUG_ASSERTIONS=false CARGO_TARGET_DIR=/root/lanes/mesh-6r1/target-noda local-scripts/with-build-slot.sh -- cargo test --release -p mesh --test all r2_byte_stability_report 2>&1 | grep -E 'panicked|identified-vertex|test result|error' | head -5
restore $C
echo "== L2: one patch withheld from the chord census — should fire on every body in RELEASE"
python3 - <<'PY'
p='crates/mesh/src/tessellate.rs'; s=open(p).read()
old="        let bad = unpaired_chord_segment(&polylines, &patch_triangles, shared_below);"; assert s.count(old)==1
open(p,'w').write(s.replace(old,"        let bad = unpaired_chord_segment(&polylines, &patch_triangles[1..], shared_below);"))
PY
local-scripts/with-build-slot.sh -- cargo test --release -p mesh --test all --no-run 2>&1 | tail -1
$(BIN) r2_byte_stability_report --test-threads=1 2>&1 | grep -E 'panicked|chord segment|test result' | head -5
restore $T
echo "== L3: does the mesh LIB unit-test target compile with debug-assertions OFF? (head, then merge base)"
CARGO_PROFILE_DEV_DEBUG_ASSERTIONS=false CARGO_PROFILE_TEST_DEBUG_ASSERTIONS=false CARGO_TARGET_DIR=/root/lanes/mesh-6r1/target-noda local-scripts/with-build-slot.sh -- cargo test -p mesh --lib --no-run 2>&1 | grep -E '^error|Finished|could not compile' | head -8
( cd review/r1-mesh6/base && CARGO_PROFILE_DEV_DEBUG_ASSERTIONS=false CARGO_PROFILE_TEST_DEBUG_ASSERTIONS=false CARGO_TARGET_DIR=/root/lanes/mesh-6r1/target-noda-base /root/lanes/mesh-6r1/local-scripts/with-build-slot.sh -- cargo test -p mesh --lib --no-run 2>&1 | grep -E '^error|Finished|could not compile' | head -8 )
echo "== restored:"; git status --short crates/mesh/src
