---
id: split-cyl-ellipse-quarter-bound-overshoots-the-tilt-at-the-band-edge
kind: issue
title: A split row on an ellipse-bounded cylinder face serves against an in-band truth: the quarter-parallelogram bound overshoots the tilt at the band edge
status: open
opened: 2026-10-08
priority: P3
cost: M
refs: [whole-turn-conic-reach-over-states-a-rim-faces-lever]
---


## What

Found in PR 4292, in fix pass 2 (review 2's n3). The split_cyl family of
`crates/geom-brep/tests/span_reach_differential.rs` now draws
ellipse-bounded patches: half of its patches, from oblique cuts with
φ ∈ [−1.2, 1.2]. Two of them serve against the truth on head. Main
serves the same two.

Both are ellipse arcs of about 2.7 rad. There the quarter-parallelogram
bound of `extent::conic_arc_reach` (`Reach::span_reach_from`) overshoots
the tilt at the band's edge:

| | truth | head |
|---|---|---|
| case 1 | 9.83·zero | 10.02·zero |
| case 2 | 9.97·zero | 10.09·zero |

So the osculation/bend rows serve graze or knife where the truth is in
band. No circle-bounded case serves against the truth: circles take the
exact reach (antipodal crest).

## The shape to give

An ellipse arc's reach from a pivot that is tight enough at the band's
edge. That could be an exact farthest-point read of the ellipse arc (a
quartic in the parameter, with certified roots), or a quarter bound
refined to the parallelogram's support. Rows: the two cases above, as
rows that fail first, plus the differential's split_cyl family at 0
served against the truth.
