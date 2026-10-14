---
id: plane-nurbs-limb-2-floor-has-an-absolute-part-that-does-not-shrink-with-the-model
kind: issue
title: plane × NURBS limb 2's delta = 0 floor reads 5.0e-14 m on a 1 mm model, about 1500x what scaling the 1 m reading gives, so the bound carries an absolute part that does not shrink with the part
status: open
opened: 2026-10-03
priority: P3
cost: M
refs: [project-eps-point-is-absolute-so-a-km-model-refuses-off-geometry, plane-nurbs-certificate-bound-does-not-refine-with-eps, 3964]
---


## What

Limb 2 of `geom_brep::plane_nurbs_limbs` (`ssi::certify::certify_branch`)
on the rigid-map fixture of
`crates/topo/tests/rigid_map_near_eps_plane_nurbs.rs`, a plane against
a rational quarter-cylinder wall with the field at δ = 0, scaled by
`s` (REACH review of PR 3964, `probes/probe_rigid_scale.rs` and
`probes/scale_results.txt` on `analysis/reach-review/3964` at
`6bb07cad48`, ε 1e-9):

| s     | δ = 0 floor  | floor / s |
|-------|--------------|-----------|
| 1e-3  | 4.998e-14 m  | 5.0e-11   |
| 1     | 3.242e-14 m  | 3.2e-14   |
| 1e3   | refused: foot-point projection did not converge | — |

Scaled from the 1 m reading, the 1 mm model's floor would be about
3e-17 m. It reads 5.0e-14 m, so the bound carries a component that does
not shrink with the model. On a millimetre part at ε 1e-12 the floor
alone would be 5% of ε, whatever the geometry. Where that component
lives is unmeasured. The candidates are an absolute constant in the
chart image's fit or in the interval enclosure's widening, the kind
`project-eps-point-is-absolute-so-a-km-model-refuses-off-geometry`
found in the projection.

The 1 km refusal is that FLUX item
(`work/flux/project-eps-point-is-absolute-so-a-km-model-refuses-off-geometry.md`):
`PROJECT_EPS_POINT` = 1e-13 m sits below the ulp of km-scale
coordinates.

`plane-nurbs-certificate-bound-does-not-refine-with-eps` is about the
fixed sample schedule (the floor does not fall with ε). This item is
about the floor not falling with the model's size.
