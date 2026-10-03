---
id: volume-door-reads-a-tiny-valid-boolean-result-wrong
kind: issue
title: mass_properties misreads a valid boolean result's volume by ~1e-15 m³ on a 10 m-span body, so tier 3 calls a 6e-19 m³ sliver NegativeVolume
status: review
opened: 2026-09-29
priority: P3
cost: M
pr: 3977
branch: reach/check7-interval
---


Filed by CONTACT-9 (review MINOR-5).

`topo::mass_properties` returns the volume of a small valid body with an
absolute error of up to ~1e-15 m³ when the body spans ~10 m. Tier 3's
signed-volume check (`NegativeVolume`) reads the same integral.

**Witness.** CONTACT-9's needle slivers are tetrahedra with exact corners
(checked against the analytic ones):
- a needle over a slab, 10 m long, dipping `500·ε`;
- a needle over a block's corner.

Measured (`mass_properties` against the exact tetrahedron):

| ε | pose | exact | read |
|---|---|---|---|
| 1e-9 | corner, 1 m edges | 5.833e-13 | 5.844e-13 |
| 1e-10 | corner, 1 m edges | 5.833e-15 | 6.229e-15 |
| 1e-11 | corner, 1 m edges | 5.8e-17 | 4.8e-16 |
| 1e-12 | slab, 1 m edges | 4.17e-19 | 2.08e-19 |
| 1e-12 | corner, 1 m edges | 5.8e-19 | −2.96e-16 |

At the last row tier 3 reports `NegativeVolume` on the valid
`intersect` result.

`crates/topo/tests/contact9_side_codes.rs` trusts the door only above
1e-12 m³ (`RESOLVED_VOLUME`) and uses the corner set below it. The row
that measured this was the review's probe, reproduced in CONTACT-9's
fix pass.

The error looks like cancellation in a divergence-theorem sum taken
about the origin, with coordinates of ~10 m. Summing about the body's
own centroid, or a vertex of it, would bound it by the body's own size.

## Evidence from REACH (`reach/door-backstop`)

The boolean volume backstop now has an interval re-derivation of the
same integral. `PastTarget::interval_volume` re-runs every closed-form
face at the interval scalar over its stored geometry and sums in
interval arithmetic. With it, the backstop's new positivity arm reads
this item's ε = 1e-12 corner sliver as straddling zero and passes it,
instead of reading it negative. Tier 3's check 7 (`plus_v_read`) and the
shell-role read (`chk_shell_volume_sign`) still decide on the
unpadded `f64` sum. They could refuse only on what that re-derivation
certifies, as the backstop does. That would cure the reading whatever
the origin's distance; recentring the sum is still the cure for the
measurement.
