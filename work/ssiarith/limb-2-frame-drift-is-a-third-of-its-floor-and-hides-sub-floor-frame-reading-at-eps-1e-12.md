---
id: limb-2-frame-drift-is-a-third-of-its-floor-and-hides-sub-floor-frame-reading-at-eps-1e-12
kind: issue
title: plane × NURBS limb 2 moves ~1e-14 m under a rotation (a third of its 3.2e-14 floor, every eps), so at eps 1e-12 the rigid-map row passes a frame-reading defect under ~3% of the bound; limb 1 resolves ~0.15% there
status: open
opened: 2026-10-03
priority: P3
cost: M
refs: [rigid-map-row-asks-a-sampled-residual-for-sub-ulp-agreement-at-eps-1e-12, plane-nurbs-limbs-under-translation-drift-with-the-coordinates]
---

## What

`crates/topo/tests/rigid_map_near_eps_plane_nurbs.rs`,
`the_certificate_re_derives_within_rounding_under_a_rotation`, reads
`geom_brep::plane_nurbs_limbs` on a plane × rational quarter-cylinder
pair seated and under 32 rotations about the origin. Limb 2
(`hull_sup`, derived in `ssi::certify::certify_branch` from
`geom_core::spline::compose::tensor::coefficient_norm_bound`) moves by
at most, in metres:

| ε     | worst `|Δ hull_sup|` | of the δ = 0 floor (3.242e-14) | of the pinned `FLOOR` (4.2e-14) |
|-------|----------------------|--------------------------------|---------------------------------|
| 1e-12 | 9.9e-15              | 0.306                          | 0.236                           |
| 1e-9  | 1.25e-14             | 0.385                          | 0.297                           |
| 1e-6  | 1.02e-14             | 0.316                          | 0.244                           |

The drift does not scale with the field, so the row states it against
`FLOOR`. The cost shows at ε 1e-12, where `FLOOR` is 4% of ε: on top of
the honest 0.24, a frame-dependent error up to about 0.76 of `FLOOR`
(3.2e-14 m, about 3% of the bound) still passes. Measured: a
per-coordinate fold over a cell's coefficients in
`coefficient_norm_bound` (≈0.1%) and a `1e-3·|z|` and `1e-2·|z|` leak
into each coefficient's norm are
all green at 1e-12 (the last reads 0.43 of the δ = 0 floor). All of them are red at 1e-9 and 1e-6.

**Limb 1 has the same limit, from rounding alone.** Limb 1 is
`|S(u*, v*) − C(t)|`, compared with the closed-form field to
`ROUNDING_ULPS` = 4 ulps of the coordinate scale √3, about 1.5e-15 m.
At ε 1e-12 the field is about 2500 of those ulps, so a frame-dependent
limb-1 error under about 0.15% of the field is not resolved there: a
`1e-3·|r.x|` leak reads 3.44 ulps (green) and dropping `r.z` reads
4.47 (red by 1.12×). This part is not a kernel defect and has no fix
in f64; it is recorded so the 1e-12 row is not read as guarding more.

**Where limb 2's drift comes from: rounding, per the translation
measurement.** Under a translation the drift follows the coordinates'
magnitude (5.6 to 8.2 floors at 1 km; see
`plane-nurbs-limbs-under-translation-drift-with-the-coordinates`),
which points at the rounding of the re-derived chart image
(`edge_nurbs::chart_image` refits the pcurve in every frame) and of the
composite's interval enclosure, not at a reading of the field. Which of
the two, and whether the rotation drift can be brought to rounding of
the coordinates (about 1e-15 m here), is the work. A drift at that
level would let the row resolve a limb-2 frame-reading defect at
ε 1e-12 as well.
