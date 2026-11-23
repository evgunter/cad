---
id: a-sketch-arc-written-by-points-is-lowered-to-a-centre
kind: issue
title: a sketch arc written by points is lowered to a centre, so a nearly flat one refuses near-tangent crossings
status: open
opened: 2026-11-22
priority: P2
cost: M
refs: [7258]
---


Filed by the ONEROUND orchestrator, from PR 7258 (what a conic stores;
Ev approved 2026-11-22; `docs/DESIGN-FORK-LOG.md` row 296).

A conic is one of two placements, fixed by its construction: held about
its centre, or about a point of the arc it serves
(`crates/geom-core/README.md`, "Float rounding"). A construction that
holds a point of the arc and whose centre nobody wrote stores that
point.

A sketch arc written by points (`Bulge`, `Via`) is such a construction,
but the profile layer lowers it to a centre: `Arc2`
(`crates/geom-core/src/arc.rs`) holds only a centre, so the arc's own
written points are recomputed from it. On a nearly flat arc the centre
is far and the conic built from it is centre-held, so it fixes a point
of its arc only to the rounding of the centre's distance.

What a person then meets, measured for PR 7258 on centre-held circles
at ε = 1e-9 (`analysis/oneround/conic-vertex-probes`): nearly tangent
crossings are refused, typed, more often as the radius grows (all 40
test poses certify at 500 m, 33 at 900 m, 28 at 1 km, none at 10 km).
Nothing measured reads wrong. The same arc held on its written start
point certifies those poses at any radius.

Owed: the profile's arc keeps a written point where the user wrote
points, down to the conic it becomes. Until then this is the main way a
person meets those refusals, and "write the arc by points on it" is not
yet a recourse.

Whether `Arc2` gains a placement of its own or the path door hands the
3D conic its point directly is the row's to decide; it changes what a
record holds.
