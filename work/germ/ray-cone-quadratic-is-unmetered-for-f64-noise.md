---
id: ray-cone-quadratic-is-unmetered-for-f64-noise
kind: issue
title: The ray x cone quadratic in point_in_solid carries no f64 noise meter; the same quadratic unmetered certifies phantom pairs from a far origin and cancels its near root
status: open
opened: 2026-09-29
priority: P3
cost: E
refs: [VERBS-CONE, line-torus-roots-may-certify-noise-when-the-line-origin-is-far]
---


## What

`crates/topo/src/boolean/solid_contain.rs` `line_cone_roots` is the
line × double-cone quadratic, shared by the ray lane (`cast_ray`'s cone
arm, which `point_in_solid` runs) and, since VERBS-CONE U1, the edge
lane. The edge lane wraps it in a noise meter
(`crates/topo/src/boolean/line_cone.rs` `line_cone_edge_roots`: the
discriminant charged its evaluation error, and each root's slack held to
the band). The ray lane calls it bare, bit-identical to before, as the
VERBS-CONE spec requires of U1.

Two measurements on the quadratic itself, through the edge lane with its
meter removed (the mutant rows in `line_cone.rs`'s tests):

- **A far parameter origin.** Lines grazing the cone by
  ±{1.6e-8 … 1e-5} m with their origin moved back along the line: at
  `1e5` m the bare quadratic certified phantom pairs on clearances and
  put dip roots up to 1.2 mm off (`a_far_origin_does_not_certify_noise`,
  which the metered lane passes).
- **A near-generator-parallel line.** `A = (d·â)² − cos²α` small but
  definite: the near root `(−B + √Δ)/A` cancels, and at two poses found
  by an exact-arithmetic search it is 2.3e-7 and 2.2e-7 m off the true
  root (`a_cancelled_near_root_is_not_certified_off_the_oracle`).

Unmeasured: whether either reaches a `point_in_solid` verdict. A ray
starts at the query point, which usually sits near the body, and the
cancellation moves a hit by ~`u·|B|/|A|`, which matters only where the
hit is within that of a trim boundary.

## Measure

Rays from query points 1e2–1e5 m from a cone face and schedule
directions within 1e-7 rad of a generator, against the exact oracle the
edge lane's rows use. If a verdict flips, give the ray lane the edge
lane's meter, or the cancellation-free root form
(`t = C/q`, `q = −(B + sign(B)√Δ)`), in a PR that re-pins
`bool2_cone_doors.rs`.

## Home

GERM, beside the VERBS-CONE root lanes.
