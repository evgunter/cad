#!/bin/bash
# R1 LILYWELD PR-1 review driver — all stages inside ONE slot hold.
S=/tmp/claude-1000/-home-evan--mngr-worktrees-kernel-verbs-b18967fd717b4619b007e5dd48192955/5b5d4856-bb77-46bd-9dc7-e24a32cf2ddd/scratchpad
L=/home/evan/.local/share/cad-work/verbs-lilyweld-r1/cad
: > "$S/lw-status.txt"
cd "$L" || exit 1
git checkout -- demos/tour/src/lily.rs 2>/dev/null

# 1. HEAD: the lily suite, release, nocapture.
( cd demos/tour && cargo test --release lily -- --nocapture ) > "$S/lw1-head.log" 2>&1
echo "1 head-lily exit=$?" >> "$S/lw-status.txt"

# 2. fmt + clippy (the k-lint step that DID run hosted).
( cd demos/tour && cargo fmt --check && cargo clippy --all-targets -- -D warnings ) > "$S/lw2-lint.log" 2>&1
echo "2 fmt+clippy exit=$?" >> "$S/lw-status.txt"

# 3. MUTATION A: ARCH_R 0.052 -> 0.0537. Weld must STILL be exact
#    (geometric necessity) and the derived globe centre must MOVE.
sed -i 's/^const ARCH_R: f64 = 0.052;/const ARCH_R: f64 = 0.0537;/' demos/tour/src/lily.rs
( cd demos/tour && cargo test --release lily -- --nocapture ) > "$S/lw3-archr.log" 2>&1
echo "3 mutate-ARCH_R exit=$? (expect SPHERE1_C pin RED, weld_circle assert GREEN)" >> "$S/lw-status.txt"
git checkout -- demos/tour/src/lily.rs

# 4. MUTATION B: wall 2's pinned kind Cone -> Sphere. The tour probe
#    must PANIC ("the wall MOVED"), proving the pin discriminates.
python3 - << 'PY'
import re
p='demos/tour/src/lily.rs'
s=open(p).read()
old="""                BooleanError::CurvedPairUnsupported {
                    op: None,
                    operand: Operand::A,
                    kind: SurfaceKind::Cone,
                    other_kind: SurfaceKind::Torus,
                    ..
                }"""
new=old.replace("kind: SurfaceKind::Cone,","kind: SurfaceKind::Sphere,")
assert s.count(old)==1, s.count(old)
open(p,'w').write(s.replace(old,new))
PY
( cd demos/tour && cargo test --release lily -- --nocapture ) > "$S/lw4-wall2.log" 2>&1
echo "4 mutate-wall2-kind exit=$? (expect RED: 'the wall MOVED')" >> "$S/lw-status.txt"
git checkout -- demos/tour/src/lily.rs

# 5. MUTATION C: retire the neck (meridian None) — the weld assert
#    must go RED, proving weld_circle is not vacuous.
sed -i 's/                Some(neck),/                { let _ = neck; None },/' demos/tour/src/lily.rs
( cd demos/tour && cargo test --release lily -- --nocapture ) > "$S/lw5-noneck.log" 2>&1
echo "5 mutate-no-neck exit=$? (expect RED at weld_circle)" >> "$S/lw-status.txt"
git checkout -- demos/tour/src/lily.rs

# 6. Tail: the tess-budget gate, reproduced as CI would run it.
scripts/tess_budget_sweep.sh target/tess-budget-fresh.csv --sizing-only > "$S/lw6-sweep.log" 2>&1
echo "6 sweep exit=$?" >> "$S/lw-status.txt"
cargo run --release -p tess-lint -- target/tess-budget-fresh.csv \
  --baseline docs/tess-budget-data/tess-budget-baseline.csv > "$S/lw7-lint.log" 2>&1
echo "7 tess-lint exit=$?" >> "$S/lw-status.txt"
echo DONE >> "$S/lw-status.txt"
