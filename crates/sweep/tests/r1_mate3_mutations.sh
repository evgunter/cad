#!/usr/bin/env bash
# MATE-3 R1 review — mutation probes (claim 3: every verdict-table arm
# has a committed row that reds under a sign flip or arm removal).
# Each block patches ONE mutation at the frozen head 428a6c39, runs the
# committed rows, and expects the listed failures. Run from the repo
# root on a clean tree; each block ends with `git checkout --` restore.
#
# Results as executed during the review (2026-09-01):
#
# M1  material_kappa_rel: drop the negation  -> 6 rows red:
#     geom-brep dihedral::kissing_cylinders_discriminate_cusp_from_slit
#     topo tier3_tests::{an_undeclared_cusp..., revert_maps...,
#       the_second_order_band..., the_pseudomanifold_gate...}
#     sweep m9_3_zip::tube_chain_rim_unions...
# M2  classify_material_pairing: swap Aligned/Opposed -> 4 dihedral unit
#     rows + 7 topo tier3 rows red (seam/lamina/cusp rows all fire).
# M3  declares_tangent_contact: ignore the class     -> 1 row red
#     (an_undeclared_cusp...: the Rest-claim sub-assertion is the pin).
# M4  declares_tangent_contact: ignore the face pair -> 1 row red
#     (same row: the foreign-pair sub-assertion is the pin).
# M5  remove the LaminaWedge push                    -> 4 rows red:
#     tier3_tests::{the_seam_is_legal..., the_second_order_band...},
#     step-export m6_6::{ball_half_flip..., cut_cylinder_conic_trim...}.
# M6  Dir::reversed as from_angle(ang + pi)          -> NOTHING red:
#     profile (280 tests), editor-core, sweep all stay green. The
#     "exact negation, never ang+pi" claim has no pin (review finding).
set -euo pipefail
run() { cargo test -p "$1" "$2" 2>&1 | grep -E "FAILED|test result" | tail -4; }

echo "== M1: material_kappa_rel without the negation"
python3 - <<'EOF'
p='crates/geom-brep/src/dihedral.rs'; s=open(p).read()
s=s.replace('    -(kappa_rel * sense_plus)\n','    kappa_rel * sense_plus\n')
open(p,'w').write(s)
EOF
run geom-brep dihedral; run topo tier3_tests; run sweep m9_3_zip || true
git checkout -- crates/geom-brep/src/dihedral.rs

echo "== M6: Dir::reversed via ang + pi (expect: all green = the gap)"
python3 - <<'EOF'
p='crates/profile/src/path.rs'; s=open(p).read()
s=s.replace('''    fn reversed(self) -> Self {
        Self::from_unit(-self.unit)
    }''','''    fn reversed(self) -> Self {
        Self::from_angle(self.ang + T::pi())
    }''')
open(p,'w').write(s)
EOF
run profile "" || true
git checkout -- crates/profile/src/path.rs
