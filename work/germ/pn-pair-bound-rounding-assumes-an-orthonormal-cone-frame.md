---
id: pn-pair-bound-rounding-assumes-an-orthonormal-cone-frame
kind: issue
title: pair_bound's rounding term R assumes the cone's u_ref is orthogonal to its axis, which no check at rest holds
status: closed
opened: 2026-10-12
closed: 2026-10-13
branch: germ/pn-apex-reach-pico-eps
pr: 4966
refs: [pn-apex-reach-fuzz-finds-a-contradiction-at-a-pico-epsilon]
priority: P2
cost: M
---


## What

`pair_bound` (`crates/geom-brep/src/intersect.rs`) step 5 charges the
served lines' own rounding as `R`. It reads `φ`, the `atan2` of the
plane normal's two components across the axis, as `5u/s` off, each
component `3u` off. That holds for an orthonormal frame. A `Cone`'s
`u_ref ⊥ axis` is not checked at rest: the at-rest frame check
(`convention::frame_margins`, called from `crates/geom/src/surfaces.rs`)
margins the frame for `Cylinder`, `Sphere` and `Torus` only. A tilt `τ = u_ref·a` moves `φ` (in
`plane_cone_section`, `v_ref = a × cone_u`, `nu`, `nv`) by about
`τ·|c|/s`, and `R` does not read it.

## Measured (PR 4646's second review)

A flat cone, `α = 1.56698`, `s = 0.0168`, `u_ref·a = −7.6e-15`. The
apex line pair is served `5.68·ε` off the real section at
`ε = 1e-12`. Main serves bit-identical lines, so this predates PR
4646. With `u_ref` re-orthogonalised against the axis, the miss drops
to `1.5e-14`.

## The shape of a fix

Either:

- charge `τ` in `R`: `|u_ref·a|·|c|/s`, times `sin α`, per unit of the
  farthest consumed point's distance from the apex; or
- re-orthogonalise the frame where it is read (`cone_u` less its
  component along `a`, renormalised) before `φ` is taken, so step 5's
  premise holds by construction.

The second also covers every other reader of the cone's frame in
this arm.

## Fixed (`germ/pn-apex-reach-pico-eps`)

The second shape: `intersect.rs` `square_seam` takes the cone's `u_ref`
less its component along the axis, renormalised, and every reader of
the cone's seam in the arm reads it through that. The readers are
`plane_cone_section`'s line pair and tangent line (`φ`, `v_ref`, the
generator directions), its `AxisNormalCircle` carrier, and
`cone_cylinder_section`'s `CoaxialCircles` carriers, whose doc claimed
the seam "⊥ the axis as stored". `pair_bound` step 5 charges the
squared seam's residual tilt (`< 6u`): `φ`'s term goes from `5u/s` to
`11u/s`, and the per-unit sum from `12·sin α` to `18·sin α`. Step 6's
coverage moves from `α ≈ 78°` to `≈ 81°`.

Rows:

- `pn_apex_pair_over_a_span::a_seam_off_square_to_the_axis_does_not_turn_the_pair`:
  the review's flat cone (`α = 1.56698`, `s = 0.0168`, tilt
  `−7.6e-15`, `ε = 1e-12`) over spans out to `6`. Pre-fix the pair is
  served `2.72·ε` off the section; post-fix `8.1e-4·ε`.
- `intersect_table::a_cones_tilted_seam_serves_its_circles_in_their_planes`:
  a seam tilted `1e-6`. Pre-fix each circle carries the tilted seam.
- the span fuzzer draws tilted seams too (pre-fix: red 1 run in 3).
