---
id: circle-torus-roots-refuse-large-circles-for-want-of-recentering
kind: issue
title: The circle x torus root door refuses circles much larger than the torus: its degree-2 harmonics cancel ~rho^4 terms
status: open
opened: 2026-09-29
priority: P3
cost: M
refs: [circle-crosses-a-torus-face-with-no-root-lane]
---

## What

`crates/topo/src/boolean/circle_torus.rs` `circle_torus_roots` builds
`F` along the carrier as degree-2 harmonics about the circle's own
centre. For a circle much larger than the torus those harmonics are
sums of terms of size `~(|C₀ − c|² + ρ²)² ~ ρ⁴`, which cancel to the
small values `F` takes where the carrier passes the tube. The noise
meter (`half_angle_roots`, `bool_circle_torus_noise` and
`bool_circle_torus_root_slack`) refuses once that rounding reaches the
band. Measured against a torus `R = 1, r = 0.25` at the default band
(`crates/topo/src/boolean/circle_torus.rs` test
`no_wrong_certified_answer_across_circle_radii`): the `f64` answers
taper from `ρ ≈ 10–15`, and at `ρ = 30` every grazing pose refuses. The
threshold scales as `ρ⁴ ≲ 10ε·r·R²/(16u)`.

The refusals are typed and sound. What they cost is circles passing
NEAR the torus. Distant circles are still cleared earlier, by the
circle rung's enclosure clearance (`reduce.rs` `circle_clearance`).

## The fix

Expand `F` about the carrier point nearest the torus rather than about
the circle's centre. Near the crossings the carrier is then a short
arc through a point at distance `~R + r` from the torus centre, and
the coefficients stay `O((R + r)⁴)` rather than `O(ρ⁴)`. The expansion
is no longer a finite trigonometric polynomial in the shifted
parameter, so this needs its own certificate for the truncation, or an
exact re-expression that keeps the magnitudes small.

## The cone lanes, the same shape (VERBS-CONE U1+U2)

- **Line × cone** (`crates/topo/src/boolean/line_cone.rs`): the
  quadratic is built about the carrier's parameter origin, so an edge
  whose `Line` origin is far from the apex carries `|w|²`-sized terms.
  Grazing lines at the default band answer 24, 16, 12 and 8 of 32 with
  the origin 0, 1e2, 1e3 and 1e4 m back along the line, and none from
  1e5 m (test `a_far_origin_does_not_certify_noise`). Recentering the
  parameter at the foot nearest the apex is exact for a line, with no
  truncation certificate.
- **Circle × cone** (`crates/topo/src/boolean/circle_cone.rs`): `Q`'s
  terms grow as `ρ²`, not `ρ⁴`. Grazing circles answer every depth up to
  `ρ = 1`, 24 of 32 at 30 m, 7 at 300 m and none from 3 km (test
  `no_wrong_certified_answer_across_circle_radii`).

## Home

GERM, beside the circle × torus root lane.

