#!/usr/bin/env bash
# R1 mutation battery over PR #1517's pins. Each mutant is applied to
# the lane tree, one targeted test run captured, then the tree is
# restored with `git checkout HEAD -- crates/` and verified clean.
# Run AFTER all digest runs (shares the lane tree).
set -uo pipefail
LANE=/root/lanes/mesh-4r1
OUT=/tmp/claude-0/r1m4/mutants
SLOT="$LANE/local-scripts/with-build-slot.sh"
mkdir -p "$OUT"
cd "$LANE"

restore() {
  git -C "$LANE" checkout HEAD -- crates/
  if ! git -C "$LANE" diff --quiet HEAD -- crates/; then
    echo "RESTORE FAILED"; exit 99
  fi
}

report() { # report <name> <expect> <exitcode> <logfile>
  echo "== $1 (expect $2): exit=$3" | tee -a "$OUT/summary.txt"
  grep -E 'test .*(FAILED|ok)$|assertion|panicked|left|right' "$4" | head -8 >> "$OUT/summary.txt" || true
}

SIZ=crates/mesh/src/sizing.rs
WALK=crates/mesh/src/walk.rs
TESS=crates/mesh/src/tessellate.rs

# ---- M1: raw() accessor + bare comparison over loop_polygon's
# coincident call. Expect: the_eps_inventory_is_pinned RED, walk.rs
# read column [1,3,1,0] -> [1,2,1,0], sizing.rs carriers still 2.
python3 - <<'EOF'
import re
p='crates/mesh/src/sizing.rs'; s=open(p).read()
s=s.replace("    /// A chosen band,", "    /// R1 MUTANT accessor.\n    pub(crate) fn raw(self) -> f64 {\n        self.0\n    }\n\n    /// A chosen band,",1)
open(p,'w').write(s)
p='crates/mesh/src/walk.rs'; s=open(p).read()
s=s.replace("if eps.coincident(d) {","if d <= eps.raw() {",1)
open(p,'w').write(s)
EOF
"$SLOT" --express 580 -- cargo test -p mesh --test all the_eps_inventory_is_pinned -- --nocapture > "$OUT/m1.log" 2>&1
report M1 RED $? "$OUT/m1.log"
restore

# ---- M2: a second raw Tol::eps() read in tessellate.rs. Expect: pin
# RED, tessellate.rs carrier 2 -> 3.
python3 - <<'EOF'
p='crates/mesh/src/tessellate.rs'; s=open(p).read()
s=s.replace("    let eps = Eps::at(tol);","    let _r1_second = tol.eps();\n    let eps = Eps::at(tol);",1)
open(p,'w').write(s)
EOF
"$SLOT" --express 580 -- cargo test -p mesh --test all the_eps_inventory_is_pinned -- --nocapture > "$OUT/m2.log" 2>&1
report M2 RED $? "$OUT/m2.log"
restore

# ---- M3: coincident edge flipped to strict `<`. Expect:
# the_band_edges_are_where_the_operations_differ RED; then poleguard at
# default band GREEN (corpus cannot see inclusivity — the PR's own
# honesty claim, demonstrated).
python3 - <<'EOF'
p='crates/mesh/src/sizing.rs'; s=open(p).read()
s=s.replace("length <= self.0","length < self.0",1)
open(p,'w').write(s)
EOF
"$SLOT" --express 580 -- cargo test -p mesh --lib sizing > "$OUT/m3a.log" 2>&1
report M3a RED $? "$OUT/m3a.log"
"$SLOT" --express 580 -- cargo test -p step-import --test all poleguard -- --test-threads=1 > "$OUT/m3b.log" 2>&1
report M3b GREEN $? "$OUT/m3b.log"
restore

# ---- M4: separates written as !coincident. Expect:
# a_poisoned_length_is_neither_near_nor_far RED.
python3 - <<'EOF'
p='crates/mesh/src/sizing.rs'; s=open(p).read()
s=s.replace("length > self.0","!self.coincident(length)",1)
open(p,'w').write(s)
EOF
"$SLOT" --express 580 -- cargo test -p mesh --lib sizing > "$OUT/m4.log" 2>&1
report M4 RED $? "$OUT/m4.log"
restore

# ---- M5: pad widened DOWN. Expect: pad_widens_upward_by_one_band RED.
python3 - <<'EOF'
p='crates/mesh/src/sizing.rs'; s=open(p).read()
s=s.replace("bound + self.0","bound - self.0",1)
open(p,'w').write(s)
EOF
"$SLOT" --express 580 -- cargo test -p mesh --lib sizing > "$OUT/m5.log" 2>&1
report M5 RED $? "$OUT/m5.log"
restore

# ---- M6: pole band genuinely widened (x2) — a DECISION change.
# Expect: poleguard at default band RED (halfcap witness flips to
# identified), demonstrating the digest corpus has teeth on the pole
# read's decision, unlike its edge inclusivity.
python3 - <<'EOF'
p='crates/mesh/src/sizing.rs'; s=open(p).read()
s=s.replace("length <= self.0","length <= self.0 * 2.0",1)
open(p,'w').write(s)
EOF
"$SLOT" --express 580 -- cargo test -p step-import --test all poleguard -- --test-threads=1 > "$OUT/m6.log" 2>&1
report M6 RED $? "$OUT/m6.log"
restore

# ---- M7 (probe, not a mutant): band-edge/NaN bitwise parity rows
# appended to sizing's test module. Expect GREEN.
python3 - <<'EOF'
p='crates/mesh/src/sizing.rs'; s=open(p).read()
probe=open('/root/lanes/mesh-4r1/review/r1-mesh4/probe-band-edges.rs').read()
body="\n".join(l for l in probe.splitlines() if not l.startswith('//'))
s=s.replace("    /// [`torus_grid_step`]'s doc claim","XPROBEX    /// [`torus_grid_step`]'s doc claim",1)
s=s.replace("XPROBEX", body+"\n\n",1)
open(p,'w').write(s)
EOF
"$SLOT" --express 580 -- cargo test -p mesh --lib r1_probe_ops > "$OUT/m7.log" 2>&1
report M7 GREEN $? "$OUT/m7.log"
restore

echo "ALL MUTANTS DONE"
git -C "$LANE" diff --stat HEAD -- crates/ | tail -1
