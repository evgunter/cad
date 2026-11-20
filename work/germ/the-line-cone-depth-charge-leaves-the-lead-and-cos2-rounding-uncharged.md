---
id: the-line-cone-depth-charge-leaves-the-lead-and-cos2-rounding-uncharged
kind: issue
title: The line × cone depth rung's charge holds the shift's rounding only: the rounding of A itself and of cos²α is charged nowhere
status: open
opened: 2026-11-20
priority: P3
cost: M
---


Found by the review of PR 7184 (GERM's one line × cone rule set).

## What

`geom_brep::curve_face::line_quadric::line_cone_roots` decides `line_quadric_cone_depth` on
`Centred::charged(disc)`, which moves `disc = B² − AC` toward zero by the rounding the shift brings
into `B` and `C` (the moved offset `w₀ + d·shift`, `rounding_charge(far·near)` and
`rounding_charge(far·|d|)`). Two roundings it does not charge:

- `A = (d·â)² − |d|²cos²α` itself, about `3u·|d|²`, which `C` multiplies in `A·C`;
- `cos²α`, read once as `half_angle.cos().powi(2)`, which every coefficient carries.

Near a generator `A` is small, so its own rounding is a large part of it. The review's exact-arithmetic
probe put the total discriminant error at up to 3.2× `charged`'s bound there, never flipping a
decided sign; GERM's own probe for PR 7184 (about 490k poses, near-generator and near-tangent, origins
out to 1e7) measured it at 0.34× the bound and found no wrong sign. The wall's and the sphere's
`charged` (`line_wall_roots`, `line_sphere_roots`) leave their `A` uncharged the same way.

## The fix owed

Charge `A`'s rounding and `cos²α`'s into `Centred::charged`, stated as the
depth rung's audit row states its margin (`docs/predicate-dimension-audit.md`),
with a row whose pose the uncharged form decides wrongly, if one exists.
