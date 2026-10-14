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

The refusals are typed. They are not the whole story at large ρ: where
the noise reading lands in the band's gap the ladder passes it, and
certifies roots past the band (below). What the refusals cost is
circles passing NEAR the torus. Distant circles are still cleared earlier, by the
circle rung's enclosure clearance (`reduce.rs` `circle_clearance`).

## The fix

Expand `F` about the carrier point nearest the torus rather than about
the circle's centre. Near the crossings the carrier is then a short
arc through a point at distance `~R + r` from the torus centre, and
the coefficients stay `O((R + r)⁴)` rather than `O(ρ⁴)`. The expansion
is no longer a finite trigonometric polynomial in the shifted
parameter, so this needs its own certificate for the truncation, or an
exact re-expression that keeps the magnitudes small.

## Home

GERM, beside the circle × torus root lane.


## Evidence (2026-10-02, the dual review of PR 3752): large circles on the shared ladder

The ladder is shared with the circle × cylinder door
(`crates/topo/src/boolean/circle_roots.rs`). On it, a circle of radius
1500 tilted to a unit wall gets roots certified 1.09–1.23e-9 m off at
ε = 1e-9, its noise reading in the gap (review 2, `topo_dr2_probe.rs`,
`dr2_gap_noise_root_error`). Recentering would shrink the terms that
put the reading there; the posture is
`circle-torus-meters-accept-an-unreadable-reading`.
