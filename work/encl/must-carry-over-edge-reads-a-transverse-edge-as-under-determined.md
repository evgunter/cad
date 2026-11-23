---
id: must-carry-over-edge-reads-a-transverse-edge-as-under-determined
kind: issue
title: must_carry_over_edge has no first-order gate: a transverse edge routed to the tangent branch is stored as a conventional chart in one surface order
status: closed
opened: 2026-09-26
priority: P1
cost: D
closed: 2026-09-26
---

## Finding

`crates/geom-brep/src/dihedral.rs`'s `must_carry_over_edge` reads
only the second-order sagitta (`tangent_second_order`) at the
certification stations. It never asks whether the join is
first-order smooth, although every caller's premise is that it is
(`crates/sweep/src/blend/surgery.rs`'s `attach_contact` tangent
branch: "a definitely-smooth join"). So a TRANSVERSE edge routed to
that branch gets the answer the jet happens to give.

Measured on PR 3270's head, with the mutant from
`tangent-parallel-certifier-passes-a-transverse-arc`
(`ContactCarrier::TransverseArc` dropped from `attach_contact`'s
`transverse` set) and a temporary probe at the `must_carry_over_edge`
call, over `fillet_h7_transverse_cap`:

- **As the fixtures order it (cap plane = `s1`, band cylinder =
  `s2`):** every cut-off arc reads `JetDeterminate`. The arc is stored
  `TangentIntersection` and the certificate refuses it at
  `TangentParallel`, which is loud and correct.
- **With the order swapped in the probe (band = `s1`):** every arc
  reads `UnderDetermined` (12 of 12). The jet's transverse direction
  is then the cylinder's flat ruling, so `κ_rel = 0` against the
  plane. The conventional `Chart` image is stored, and the body passes
  `validate_geometric`. The rod, D-profile and wrong-radius rows fail
  only at their own `EdgeDescription::Intersection` assertion.
  `one_crease_alone_carves_at_half_the_prism`, which does not assert
  the description, passes outright.

The chart image is not a false description (the locus is on both
surfaces), but it is the weaker one, stored for an edge whose honest
intrinsic description is `Intersection`. Nothing downstream notices,
and it is silent in exactly one surface order.

## Fix shape

Gate `must_carry_over_edge` on a first-order smooth reading
(`classify_dihedral` → `Smooth`) at its stations, and refuse typed
(or answer a fourth verdict) where the join is definitely transverse.
That spends new decisions, so the K-REPORT runbook applies to
whatever reading it adds. The surgery routing is BAND/CARVE ground
(`attach_contact`); the rule is `geom-brep`'s, which no program's
`paths` cover. Filed here beside the unit that found it; the owner
places it.
