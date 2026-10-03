---
id: limb-2-frame-drift-is-a-third-of-its-floor-and-hides-sub-floor-frame-reading-at-eps-1e-12
kind: issue
title: plane × NURBS limb 2 moves ~1e-14 m under a rigid map (a third of its 3.2e-14 floor, every eps), so at eps 1e-12 a frame-reading defect under ~2% of the bound is invisible
status: open
opened: 2026-10-03
priority: P3
cost: M
refs: [rigid-map-row-asks-a-sampled-residual-for-sub-ulp-agreement-at-eps-1e-12]
---


## What

`crates/topo/tests/rigid_map_near_eps_plane_nurbs.rs`,
`the_certificate_re_derives_within_rounding_under_the_map`, reads
`geom_brep::plane_nurbs_limbs` on a plane × rational quarter-cylinder
pair seated and under 32 rotations. Limb 2 (`hull_sup`, derived in
`ssi::certify::certify_branch` from
`geom_core::spline::compose::tensor::coefficient_norm_bound`) moves by
at most, in metres:

| ε     | worst `|Δ hull_sup|` | as a fraction of the δ = 0 floor (3.242e-14) |
|-------|----------------------|----------------------------------------------|
| 1e-12 | 9.9e-15              | 0.306                                        |
| 1e-9  | 1.25e-14             | 0.385                                        |
| 1e-6  | 1.02e-14             | 0.316                                        |

The drift does not scale with the field, so the row now states it
against the floor. The cost shows at ε 1e-12, where the floor is 3% of
ε: a cell-level per-coordinate fold injected into
`coefficient_norm_bound` (per-coordinate maxima over a cell's
coefficients, folded to a Euclidean norm) moves limb 2 by about 0.1%,
which is 30× the floor at ε 1e-9 and 3e4× at 1e-6 (the row goes red)
but about 1e-15 m at 1e-12, under the honest drift (the row stays
green). A frame-reading defect in limb 2 smaller than about a third of
the floor is invisible at that ε.

**Unmeasured: where the 1e-14 comes from.** The floor is what the
bound reads with no field, so the candidates are the re-derived
chart image (`edge_nurbs::chart_image` refits the pcurve in every
frame) and the interval widening of the composite. Measuring which,
and whether the drift can be brought to rounding of the coordinates,
is the work; a drift at rounding would let the row resolve a
frame-reading defect at ε 1e-12 too.
