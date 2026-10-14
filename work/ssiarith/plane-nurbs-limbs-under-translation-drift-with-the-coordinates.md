---
id: plane-nurbs-limbs-under-translation-drift-with-the-coordinates
kind: issue
title: under a translation the plane × NURBS limbs drift with the coordinates' magnitude (limb 2 at 5.6-8.2 floors under 1 km), so the rigid-map row covers rotations about the origin only
status: open
opened: 2026-10-03
priority: P3
cost: M
refs: [limb-2-frame-drift-is-a-third-of-its-floor-and-hides-sub-floor-frame-reading-at-eps-1e-12, project-eps-point-is-absolute-so-a-km-model-refuses-off-geometry, 3964]
---


## What

`crates/topo/tests/rigid_map_near_eps_plane_nurbs.rs` checks the
plane × NURBS certificate (`geom_brep::plane_nurbs_limbs`) under 32
rotations about the origin, and under no translation. A rotation keeps
every coordinate's magnitude, so the row's widths hold. Limb 1 is
within `ROUNDING_ULPS` of the closed-form field, in ulps of the seated
wall's coordinate scale, and limb 2 moves by at most the pinned
`FLOOR`. A translation is the other half of a rigid map, and neither
width is stated for it.

**Measured** (REACH review of PR 3964, `probes/probe_rigid_scale.rs`
and `probes/scale_results.txt` on `analysis/reach-review/3964` at
`6bb07cad48`, ε 1e-9). Same rotations, plus a translation `tr` along
(0.37, −0.81, 0.45), model scale `s`:

- **Limb 1** stays at most 1.74 ulps of the IMAGE's coordinate scale at
  every scale and translation tried, so the closed-form oracle is
  honest under translation if the ulp is read from the image. Read from
  the seated wall, as the row does, it is 8 to 11 ulps at `tr = 10·s`
  and 690 to 870 ulps at `tr = 1e3·s`.
- **Limb 2** moves by 0.26 to 0.40 of the δ = 0 floor up to
  `tr = 10` m, and by **5.6 to 8.2 floors at `tr = 1` km** (s = 1).
  Several images refuse at 1 km with "foot-point projection did not
  converge", which is
  `work/flux/project-eps-point-is-absolute-so-a-km-model-refuses-off-geometry.md`.

## The work

A translation row needs both widths stated against the image's
coordinate scale. Limb 1 is ready to be. Limb 2 has no derived width
for it yet: its drift follows the coordinates' magnitude, which points
at rounding in the re-derived chart image and the composite's
enclosure (`limb-2-frame-drift-is-a-third-of-its-floor-and-hides-sub-floor-frame-reading-at-eps-1e-12`).
Once that drift is stated as a function of the coordinate scale, add
translations of `10·s` and larger to the row, with both limbs' widths
read from the image.
