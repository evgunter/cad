---
id: sense-sign-doors-take-the-bit
kind: unit
title: The five bare-T sense-sign doors take the bit, and sphere's mixed sign splits into a bit and a Sign
status: closed
opened: 2026-09-15
branch: scalar/sense-sign-doors
pr: 2649
closed: 2026-09-15
---


## What

`D6`'s ruling (PR 2457), first unit. `geom_brep::classify_material_pairing`,
`material_kappa_rel`, `rim_wedge::classify_shared_rim`, `curved_face`
and its private `sphere` take the sense as `bool` instead of a bare
`T` ±1; the multiply becomes a conditional negation (exact in IEEE, so a
D9 bit-identity differential over the callers' suites is the pin).
`sphere`'s `s_f` is two things spelled as one `T` — the face sense on
one branch and a decided rim side (`t_sign(Sign)`, which can be zero) on
the other — and splits into a bit and a `Sign`; the unit checks nothing
relied on the zero. The `dihedral.rs` tests that pass `1.0`/`-1.0`
literals move to the bit. Announce to TOPO (`topo/src/{census,validate,
boolean/mod,props}.rs`), PROPS (`geom-brep/src/props/curved.rs`) and
BOOL. Full v6 dual.

## Closed (2026-09-15) — PR 2649

`classify_material_pairing`, `material_kappa_rel`, `classify_shared_rim`,
`curved_face` and its private `sphere` take `sense: bool`; every caller
passes `face.sense`. `sphere`'s mixed `s_f` is `SphereFluxSide { Sense(bool),
Rim(Sign) }` (kept beside `MaterialSign` on purpose: that one is what the
boundary alone encodes, derived without the bit so tier 3 can cross-check
the two); `linear_rim_side` is the one home of "Zero refuses here" and
neither trig site guards. `classify_material_pairing` mints
`OutwardNormal::from_chart` internally. The missing-face default in
`verify_tangent_declaration` is gone (BOOL's row closed); `face_of` names
the two refusals apart. Receipt: bit-identical at `f64` on every non-NaN,
non-zero value; NaN sign, signed zero and `Interval` ±1-product padding
differ, head tighter, no verdict moves (pinned). The `face_normal.rs`
census loses its four per-file zero rows (`PINNED` 10 → 6). A rim whose
two faces carry different senses now has a row (R1's surviving mutant
killed); an e2e row pins that a one-band-reversed ball measures volume
`0.0` silently and is caught only by tier 3 (PROPS' residual row
extended). The `blend/arms.rs` `trace` ±1 field is handed to the second
unit. Reviews: dual, both APPROVE WITH FIXES, no MAJOR; twelve fix-pass
items taken, none declined. Rows: `work/issues/anti-re-fork-row-reds-on-a-doc-pointer-to-the-door.md`
filed; PROPS' `m6-sense-gate-recorded-residuals` and this program's
second unit extended.
