#!/usr/bin/env bash
# MATE-3 — the UNION mutation matrix, re-run over the fix-pass tree.
#
# The union of R1's six mutations and R2's ten, deduplicated, plus the
# two the fix pass created a claim for (M11, M12). Every mutation
# flips ONE arm of the verdict table or one exactness promise, runs the
# committed rows, and records which go red. A mutation that reds
# NOTHING is a claim with no pin behind it.
#
# Run from the repo root on a clean tree. Each block restores itself.
set -uo pipefail

D=crates/geom-brep/src/dihedral.rs
V=crates/topo/src/validate.rs
P=crates/profile/src/path.rs

restore() { git checkout -- "$D" "$V" "$P"; }
trap restore EXIT

# Runs the suites that can see this arm; prints the failing row names.
run() {
  local out
  out=$(
    cargo test -q -p geom-brep --lib 2>&1
    cargo test -q -p topo --lib 2>&1
    cargo test -q -p profile 2>&1
    cargo test -q -p step-export --test all 2>&1
    cargo test -q -p sweep --test all m9_3 2>&1
  )
  local n
  n=$(printf '%s\n' "$out" | grep -cE "^(test result: FAILED|error\[)" || true)
  printf '%s\n' "$out" | grep -E "^ +[a-z0-9_]+::[a-z0-9_:]+$" | sort -u | sed 's/^/      RED /'
  if [ "$n" = "0" ]; then echo "      (nothing red)"; fi
}

mutate() { # name, file, python-replacement
  echo "== $1"
  python3 - "$2" <<PYEOF || { echo "   APPLY FAILED"; restore; return; }
import sys
p = sys.argv[1]
s = open(p).read()
a = $3
b = $4
assert a in s, "anchor missing"
open(p, 'w').write(s.replace(a, b, 1))
PYEOF
  run
  restore
}

mutate "M1 pairing sign flip (Aligned <-> Opposed)" "$D" \
  "'''        Sign::Positive => Ok(MaterialPairing::Aligned),
        Sign::Negative => Ok(MaterialPairing::Opposed),'''" \
  "'''        Sign::Positive => Ok(MaterialPairing::Opposed),
        Sign::Negative => Ok(MaterialPairing::Aligned),'''"

mutate "M2 material_kappa_rel drops the negation" "$D" \
  "'    -(kappa_rel * sense_plus)'" "'    kappa_rel * sense_plus'"

mutate "M3 material_kappa_rel ignores sense_plus" "$D" \
  "'    -(kappa_rel * sense_plus)'" "'    -kappa_rel'"

mutate "M4 declaration ignores the CLASS" "$V" \
  "'''        d.class == crate::contact::ContactClass::Tangent
            && ((d.a == a && d.b == b) || (d.a == b && d.b == a))'''" \
  "'        ((d.a == a && d.b == b) || (d.a == b && d.b == a))'"

mutate "M5 declaration ignores the face PAIR" "$V" \
  "'''        d.class == crate::contact::ContactClass::Tangent
            && ((d.a == a && d.b == b) || (d.a == b && d.b == a))'''" \
  "'        d.class == crate::contact::ContactClass::Tangent'"

mutate "M6 declaration always true" "$V" \
  "'    declarations.iter().any(|d| {'" \
  "'''    if true {
        return true;
    }
    declarations.iter().any(|d| {'''"

mutate "M7 the lamina refusal removed" "$V" \
  "'        MaterialArmOutcome::Lamina => Some(ValidationError::LaminaWedge { edge }),'" \
  "'        MaterialArmOutcome::Lamina => None,'"

mutate "M8 declared arm covers Cusp only (Slit legal undeclared)" "$D" \
  "'        matches!(self, Self::Cusp | Self::Slit)'" \
  "'        matches!(self, Self::Cusp)'"

mutate "M10 cusp door re-derives the ray as ang + pi" "$P" \
  "'        Self::from_unit(-self.unit)'" \
  "'        Self::from_angle(self.ang + T::pi())'"

mutate "M11 the pairing SPLIT falls silent (item 6's silence, restored)" "$V" \
  "'''        MaterialArmOutcome::Split { predicate } => Some(ValidationError::SliverDihedral {'''" \
  "'''        MaterialArmOutcome::Split { predicate: _ } if true => None,
        MaterialArmOutcome::Split { predicate } => Some(ValidationError::SliverDihedral {'''"

mutate "M12 a split END resolves as the first end seen (item 6, fold half)" "$V" \
  "'''                    Some(wedge) if !side_mixed => MaterialArmOutcome::Wedge(wedge),'''" \
  "'''                    Some(wedge) => MaterialArmOutcome::Wedge(wedge),'''"

echo "== done"
