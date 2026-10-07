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
nearest points on the link carries `gap/2` to each. Where the circles are
separated that is the least any point can carry; where they overlap by
`gap` they cross at two exact points, but those sit about `√(gap·r)` off
the link and are ill-conditioned in the gap, so a decided tangency's one
point is still taken on the link.

## Hits outside the fillet

Grepped for `powi(2) - … .powi(2)) / (` and the `d * d + r1 * r1 - r2 * r2`
spelling across `crates/`; the src hits that take the foot as a decided
tangent's one point, each its owner's to weigh:

- `crates/profile/src/seg.rs`, the arc/arc contact classifier's internal
  `carrier_circles_internal` Zero arm (PATHS). Filed on its own as
  `an-internal-tangency-reads-its-spans-at-the-radical-foot-off-both-arcs`
  (P2): near-concentric carriers put the foot centimetres off both
  arcs, and the chordal `arc_span` reading of that point refuses
  `TangentialContact` between arcs that are 1–3 cm apart, so the
  radial place does decide a verdict there. The external arm takes
  `c₁ + û·r₁` and is already gap-optimal. **This point is read as a
  location**: `crates/profile/src/validate.rs` `judge_pair` discounts a
  `Touch` only where `seg::coincident("contact_at_shared_vertex",
  contact.point, v)` is Zero. Measured with a probe (two arcs meeting at
  a declared joint `v`, their carriers through `v` and decided
  internally tangent at a margin of 0.5ε, at ε = 1e-9 and 1e-6, carrier
  radii 1 against 0.99, 0.999 and 0.5): where the arcs meet end to start
  the contact falls outside one arc's span and the loop validates; where
  they form a cusp at `v` and the carriers' second crossing falls on
  both arcs, the contact lies on both and the loop refuses
  `TangentialContact`. That refusal is the contact's ANGLE, not its
  radial place: two carriers through `v` decided tangent at margin `m`
  really cross again about `r·√(2m/Δr)` from `v`, the foot sits midway,
  and its radial error `m·r/Δr` is smaller than that by `√(m/2Δr)`. So
  the radial amplification this issue is about changes this consumer's
  verdict only where `Δr` is within a few ε of the gap, and a
  gap-optimal point would not have validated the probe's cusp.
- `crates/geom-brep/src/intersect.rs`, `sphere_sphere_section`'s
  `TangentPoint` on both Zero arms (GERM/REACH/TANG). Its one in-tree
  consumer, `crates/topo/src/boolean/join.rs` `pair_section_frame`,
  matches `TangentPoint(_)` and discards the point (a typed desync), so
  nothing reads its place today; only `geom-brep`'s own
  `intersect_table` test reads it.
- `crates/topo/src/boolean/carrier_cross.rs`, the circle crossing's
  `apart`/`nested` Zero return of `foot` (CLEAVE/HONE).
- `crates/topo/src/validate.rs`, the circle-circle edge-pair candidates'
  tangent arm (RESTFRONT).
- `crates/topo/src/boolean/join.rs` `parallel_radical_plane`, the same
  formula for two parallel cylinders' radical plane. One reviewer
  judged it crossing-only, because the frame dispatch refuses tangent
  walls before it is asked; not re-verified here.

All of them put the point on the link line, so its ANGLE about either
centre is the link's; a reader that only span-checks that angle sees
nothing. What moves is its radial place, which matters wherever the
point is used as a location. The location reader found, above, is
dominated by the angle; the span reading of the same point is not, and
has its own item. That keeps this row at P3 for the other hits.
