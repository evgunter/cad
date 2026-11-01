#!/bin/bash
set -u; cd /root/lanes/mesh-6r1; L=/root/lanes/mesh-6r1; S=$L/local-scripts/with-build-slot.sh; R=$L/review/r1-mesh6
export CARGO_TARGET_DIR=$L/target
step(){ echo; echo "##### $(date +%T) $*"; }
step "A revert measurement mutant"
git show HEAD:crates/mesh/src/curved.rs > crates/mesh/src/curved.rs; git show HEAD:crates/mesh/src/tessellate.rs > crates/mesh/src/tessellate.rs
git status --short crates/; git diff --stat HEAD -- crates/ | tail -1
step "B r2_bytes HEAD dev + release (default eps)"
$S -- cargo test -p mesh --test all r2_byte_stability_report -- --nocapture --test-threads=1 2>&1 | grep -E '^[a-z_0-9.]+ d=' > $R/r2_head_dev.txt; wc -l < $R/r2_head_dev.txt
$S -- cargo test --release -p mesh --test all r2_byte_stability_report -- --nocapture --test-threads=1 2>&1 | grep -E '^[a-z_0-9.]+ d=' > $R/r2_head_rel.txt; wc -l < $R/r2_head_rel.txt
step "C base worktree: r2_bytes dev + release, cross-build cost x2 (release)"
( cd $R/base && export CARGO_TARGET_DIR=$L/target-base
  $S -- cargo test -p mesh --test all r2_byte_stability_report -- --nocapture --test-threads=1 2>&1 | grep -E '^[a-z_0-9.]+ d=' > $R/r2_base_dev.txt; wc -l < $R/r2_base_dev.txt
  $S -- cargo test --release -p mesh --test all r2_byte_stability_report -- --nocapture --test-threads=1 2>&1 | grep -E '^[a-z_0-9.]+ d=' > $R/r2_base_rel.txt; wc -l < $R/r2_base_rel.txt
  for r in 1 2; do CAD_S65_COST=1 $S -- cargo test --release -p mesh --test all issue897_guard_cost_report -- --nocapture --test-threads=1 > $R/ab/xb_base_r$r.txt 2>&1; done )
for r in 1 2; do CAD_S65_COST=1 $S -- cargo test --release -p mesh --test all issue897_guard_cost_report -- --nocapture --test-threads=1 > $R/ab/xb_head_r$r.txt 2>&1; done
echo "r2 diffs (expect empty):"; diff $R/r2_head_dev.txt $R/r2_base_dev.txt && echo "dev: 21/21 identical"; diff $R/r2_head_rel.txt $R/r2_base_rel.txt && echo "rel: 21/21 identical"; diff $R/r2_head_dev.txt $R/r2_head_rel.txt && echo "head dev==rel"
step "D mutation battery"
python3 $R/mutants.py
step "E liveness (release, and debug-assertions off)"
bash $R/liveness.sh
step "F bounds_census at head, then with a generic array-in-angle-list signature planted"
$S -- cargo test -p geom-core --test all bounds_census 2>&1 | grep -E 'test result|FAILED|panicked' | head -3
python3 - <<'PY'
p='crates/mesh/src/tessellate.rs'; s=open(p).read()
old="fn unpaired_chord_segment(\n    polylines: &[&[u32]],\n    patch_triangles: &[&[[u32; 3]]],"
assert s.count(old)==1
open(p,'w').write(s.replace(old,"fn unpaired_chord_segment<'a>(\n    polylines: &[&[u32]],\n    patch_triangles: impl IntoIterator<Item = &'a [[u32; 3]]>,"))
PY
$S -- cargo test -p geom-core --test all bounds_census 2>&1 | grep -E 'test result|FAILED|panicked|angle|stops|cannot' | head -6
git show HEAD:crates/mesh/src/tessellate.rs > crates/mesh/src/tessellate.rs
step "G three-eps battery, -p mesh (dev)"
for e in default 1e-6 1e-12; do
  if [ $e = default ]; then env -u CAD_TOLERANCE_EPS $S -- cargo test -p mesh 2>&1 | grep -E 'test result|FAILED' > $R/eps_$e.txt; else CAD_TOLERANCE_EPS=$e $S -- cargo test -p mesh 2>&1 | grep -E 'test result|FAILED' > $R/eps_$e.txt; fi
  echo "eps=$e: $(grep -c 'test result: ok' $R/eps_$e.txt) ok-blocks, $(grep -c FAILED $R/eps_$e.txt) FAILED lines"; grep 'test result' $R/eps_$e.txt | head -3
done
step "H final byte-identity check of PR-owned files"
git status --short crates/ docs/ Cargo.toml; git diff --stat HEAD -- crates/ | tail -1; echo "phase2 done"
