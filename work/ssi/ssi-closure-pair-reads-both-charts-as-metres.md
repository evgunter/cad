---
id: ssi-closure-pair-reads-both-charts-as-metres
kind: issue
title: ssi: the march's closure pair reads the ℝ⁴ state, both charts, where its margins claim the carrier in metres
status: open
opened: 2026-10-03
priority: P3
cost: E
---

Found by the sweep of `ssi-step-rungs-mix-state-and-carrier-units`
(same class: a state-space quantity read as the carrier's metres).

## What

`march`'s closure pair (`crates/geom-brep/src/ssi/march.rs`, "the
closure pair", ~:1064) states its margins in metres but reads them off
the state:

- **`ssi_closure_return`**'s margin is `h_meters − back`, with
  `back = distance_meters(next, seed)`. `distance_meters` (~:1219) sums
  `(Δxᵢ·scaleᵢ)²` over all four ℝ⁴ coordinates, so on the plane × NURBS
  lane it counts the plane chart's displacement and the wall chart's,
  each about the 3-D one: `back` reads about √2 times the carrier's
  distance to its seed, and each wall axis by its own chart speed rather
  than the metric.
- **`ssi_closure_tangent`** reads `cos φ = d₁·t₀` between unit ℝ⁴ state
  tangents, which hold the wall pcurve's direction too, not between the
  carrier's tangents.

Nothing wrong has been measured: a lap's nearest sample lies within
about `h/2` of the seed, which still closes at √2 over, and the tangent
test reads `O(1)` angles. The margins are mis-stated, not yet wrong.

## Fix

Read `back` as `(point(next) − point(seed)).norm()`, and the angle
between the carrier's tangents (`carrier_jet`'s `C′`, the chart-A
velocity `tangent_speed` measures). On the ℝ³ lane the state is the
point and the scales are 1, so both are the same numbers there. Pin a
loop whose closure distance falls between `h/√2` and `h`.
