---
id: circle-plane-first-harmonic-has-three-hand-built-copies
kind: issue
title: The circle x meridian-plane first harmonic is hand-assembled in three places with no shared door and an unaudited noise charge
status: open
opened: 2026-10-06
priority: P2
cost: M
---

Found by the dual review of PR 4044 (`analysis/reach-dual/4044-r1`,
style Q1; `-r2`, style Q1).

## What

A circle arc against a plane through a sphere's centre is the first
harmonic `g·(C − c) + ρ(g·û) cos θ + ρ(g·v̂) sin θ`. Three sites build
its `circle_roots::FirstHarmonic` by hand, line for line:

- `boolean::ops`'s `apply_cut_ins` (ops.rs ~3840-3860): the face's
  boundary arcs against the cut's meridian plane;
- `boolean::sphere_region`'s `SphereFaceRegion::ray_roots`
  (sphere_region.rs ~392-420): the boundary arcs against a geodesic
  ray's plane;
- and, for the same algebra against a sphere or a cylinder rather than
  a plane, `geom_brep::implicit`'s `circle_sphere_harmonic`
  (implicit.rs ~1359) and `conic_cylinder_harmonics` (implicit.rs
  ~1080), which do carry a frame-defect charge.

The two plane sites charge `rounding_charge(|C − c| + ρ)` to both ends
and nothing to the phase (`phase_noise: T::zero()`), and neither charges
the frame defect of a `û`, `axis × û` pair that is not orthonormal.
`geom_brep`'s two builders do charge it
(`CircleSphereHarmonic::frame_error`).

## Asked

One circle × plane door beside `circle_sphere_harmonic` that both plane
sites call, with its noise charge audited against the frame defect and
the phase. A missing charge here is a root certified where the arc's
stored frame leaves it undecided.
