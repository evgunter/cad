---
id: decided-tangent-point-is-the-radical-foot
kind: issue
title: decided-tangent contact points taken at the radical-line foot sit off both circles by gap*(r1+r2)/d, not the gap
status: open
opened: 2026-10-07
priority: P3
cost: E
---


Found by the fillet-radius P0 (`profile-fillet-radius-off-at-eps-1e-6`),
which fixed the one instance that stores geometry: the fillet centre on a
decided offset tangency (`crates/profile/src/sugar.rs`,
`ArcCarrier::offset_circles`).

## The shape

Two circles (or spheres) decided tangent inside the band, `|gap| < ε`, and
the one contact point taken at the radical-line foot `c₁ + û·a`,
`a = (d² + r₁² − r₂²)/2d`. When the circles miss tangency by `gap`, that
foot sits `gap·r₂/d` off circle 1 and `gap·r₁/d` off circle 2: together
`gap·(r₁ + r₂)/d`. On internal tangency `d ≈ |r₁ − r₂|`, so near-equal
radii amplify the gap by far more than its own size: by the formula 16x
on the `1/0.9/0.01` lens of the fillet row
`near_half_turn_and_extreme_sweep_fillets_meet_the_oracle_at_both_scalars`,
and up to 4.1x measured over the fillet fuzz's natural draws at ε = 1e-4,
both before the fix. The point midway between the circles'
nearest points on the link carries `gap/2` to each, which is the least.

## Hits outside the fillet

Grepped for `powi(2) - … .powi(2)) / (` and the `d * d + r1 * r1 - r2 * r2`
spelling across `crates/`; the src hits that take the foot as a decided
tangent's one point, each its owner's to weigh:

- `crates/profile/src/seg.rs`, the arc/arc contact classifier's internal
  `carrier_circles_internal` Zero arm (PATHS). The external arm takes
  `c₁ + û·r₁` and is already gap-optimal.
- `crates/geom-brep/src/intersect.rs`, `sphere_sphere_section`'s
  `TangentPoint` on both Zero arms (GERM/REACH/TANG); the point is
  returned as the section, so it is the most consequential of these.
- `crates/topo/src/boolean/carrier_cross.rs`, the circle crossing's
  `apart`/`nested` Zero return of `foot` (CLEAVE/HONE).
- `crates/topo/src/validate.rs`, the circle-circle edge-pair candidates'
  tangent arm (RESTFRONT).

All four put the point on the link line, so its ANGLE about either centre
is exact; a reader that only span-checks that angle sees nothing. What
moves is its radial place, which matters wherever the point is used as a
location. Not measured at these sites: whether any consumer reads it so.
