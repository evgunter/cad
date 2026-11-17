---
id: an-analytic-outer-loops-whole-span-boxes-never-tighten-under-the-ring-reader
kind: issue
title: the ring door's analytic reader spends its work bound and answers undecided against an outer loop row whose chart box does not tighten (IsoArc, Fitted, General, Spiric)
status: open
opened: 2026-11-14
priority: P2
---


Found in review of `a-rings-lift-is-placed-against-its-outer-loop-in-one-place` (branch `tesslane/ring-placement`).

## What it is

The ring door's analytic reader (`crates/topo/src/pcurves_rings.rs`, `Outer::side` and `Outer::parity`) clears each piece of the outer loop against a point by the piece's chart box (`geom_brep::Pcurve::chart_box`), halving a piece it cannot clear. Some boxes do not shrink when their span is halved:
- an `IsoArc`'s box is its whole segment's;
- a `Fitted` or `General` image's box is its whole control net's;
- a `Spiric` image's box is its whole period's in `f`.

So a point inside such a box is never cleared. The reading spends `PIECE_VISITS` (16384 decides) and answers `Near`, and the door answers `RingPlacementError::Undecided`. The answer is typed, but no measured case produces it: the 24 measured outer loops (`crates/mesh/tests/lift_probes.rs`) are all `Harmonic`, whose boxes tighten.

## What a tightening box takes

A per-span box for each of those forms: a sub-span's box that shrinks with the sub-span.
- `Fitted` and `General`: the Bézier pieces of the net over the span, as `spline_trim`'s `bezier_pieces` cuts them (its control boxes tighten under subdivision).
- `IsoArc`: the segment between its two ends over the sub-span, since `g` is monotone in `t`.
- `Spiric`: `f`'s range over the sub-span of `cos t`, in place of its whole-period range.

`work/tesslane/one-chord-rule-over-each-boundary-images-speed-bound.md` already owes "a box over a parameter span" per image form beside `Pcurve::eval`. That unit's boxes close this row if they are restriction-tight, so this row is evidence for it and may close with it.

## A pose that reaches it (2026-11-14, TESSLANE unit 3)

The donut `(2, 0.5)` about `y` less the slab `(−0.6, 0.6) × (−1, 1) ×
(1.2, 2.8)` has spirics on its torus faces' outer loops; a stub
`(−0.02, 0.02) × (−0.1, 0.1) × (2.3, 3)` turned 0.33 about `y` rings one
of them, and in both unions and the donut ∖ stub `Body::face_lift`
answers `Undecided { cause: Some(Indeterminate { margin ≈ 1.0e-8,
predicate "ring_place_clear" }) }`, so the mesh refuses `DomainUnread`
at every δ. The same stub turned 0.5, −0.5, 1.5 or 3.0 places and
meshes. Pinned by
`reach_probes::ringed_curved_faces_that_still_refuse_answer_by_cause`.
