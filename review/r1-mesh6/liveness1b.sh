#!/bin/bash
# L1b: the seam/pole census firing END TO END on issue 678's own witness with the pole floor removed,
# (i) release as the manifest ships it, (ii) release under the post-publish manifest (debug-assertions off).
set -u; cd /root/lanes/mesh-6r1; S=local-scripts/with-build-slot.sh; C=crates/mesh/src/curved.rs
git show HEAD:$C > $C
python3 - <<'PY'
p='crates/mesh/src/curved.rs'; s=open(p).read()
old="    if has_pole && nu == 2 { 3 } else { nu }"; assert s.count(old)==1
open(p,'w').write(s.replace(old,"    let _ = has_pole; nu"))
PY
T=apex_wedges_never_size_to_a_single_azimuth_column
echo "== (i) release, manifest as shipped (debug-assertions = true):"
CARGO_TARGET_DIR=/root/lanes/mesh-6r1/target $S -- cargo test --release -p mesh --test all $T 2>&1 | grep -E 'panicked|identified-vertex|NonManifold|test result' | head -4
echo "== (ii) release, CARGO_PROFILE_RELEASE_DEBUG_ASSERTIONS=false:"
CARGO_PROFILE_RELEASE_DEBUG_ASSERTIONS=false CARGO_TARGET_DIR=/root/lanes/mesh-6r1/target-noda $S -- cargo test --release -p mesh --test all $T 2>&1 | grep -E 'panicked|identified-vertex|NonManifold|test result' | head -4
git show HEAD:$C > $C; echo "restored:"; git status --short crates/
