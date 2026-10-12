---
id: pn-pair-bound-rounding-assumes-an-orthonormal-cone-frame
kind: issue
title: pair_bound's rounding term R assumes the cone's u_ref is orthogonal to its axis, which no check at rest holds
status: open
opened: 2026-10-12
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
