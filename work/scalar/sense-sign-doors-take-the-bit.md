---
id: sense-sign-doors-take-the-bit
kind: unit
title: The five bare-T sense-sign doors take the bit, and sphere's mixed sign splits into a bit and a Sign
status: open
opened: 2026-09-15
branch: scalar/sense-sign-doors
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
