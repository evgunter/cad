---
id: a-kept-rim-off-its-sphere-is-read-on-its-own-sphere-so-member-orders-diverge
kind: issue
title: The fan form reads a kept rim that stands off its sphere on the circle's own sphere, so the tube-dome union's member orders answer differently at eps 1e-12
status: open
opened: 2026-10-11
priority: P2
cost: M
refs: [a-settled-declared-coincidence-crosses-a-tight-volume-bound]
---


Found on PR 4573 (`flux/sphere-arm`), by its fix pass's verifier; the
asymmetry was accepted there under a pinned row, and this is its home.

## Measured

`crates/sweep/tests/pi_seam_and_kiss_through_the_boolean.rs`,
`a_rim_offset_inside_the_zero_band_answers_alike_in_both_member_orders`:
a tube of radius `R + k·zero` (`R = 1`, `H = 2`) unioned with a dome
whose base disc has radius `R`, discs declared `Rest`, `k ∈ {−¼, −½}`.
The union keeps the first operand's rim circle and certifies the
second operand's row against it.

At ε = 1e-12, both `k`:

| order | kept rim | outcome |
|---|---|---|
| 0 (tube, dome) | the tube's, `R + k·zero`, standing `2.5e-13` (`k = −¼`) off the dome's sphere | builds; volume equals `vol(A) + vol(B)` to the last bit, `9e-16` from the closed form |
| 1 (dome, tube) | the dome's, `R`, standing off the tube's wall | refuses `ResultVolumeImplausible { which: "vol(A ∪ B) ≤ vol(A) + vol(B)" }`, crossing the bound by `1.05e-12` |

At the default ε and at 1e-6 both orders refuse at that bound alike: the
standoff is past the fan's agreement band, so Gauss–Bonnet's value
stands and carries the standoff into the volume.

## Why

`geom_brep::props::curved::solid_angle`
(`crates/geom-brep/src/props/curved.rs:2477-2479`) reads each arc's
`1 − d` as `ρ²/(offset² + ρ²)/(1 + d)`, with `d` normalised by the
CARRIER's own reach `√(offset² + ρ²)`, not the sphere's radius. For a
circle on the sphere the two agree. For a kept rim that stands off the
sphere inside the zero band they do not: the fan reads the face as if
its rim lay on the circle's own sphere, which takes the standoff out of
the dome face's flux. Where the fan and Gauss–Bonnet agree within
`FAN_AGREEMENT_SR` the fan is taken, so at ε = 1e-12 (standoff
`2.5e-13`) order 0's volume carries no standoff at all, while order 1's
standoff sits on the tube's wall, which the cylinder's form reads, and
crosses the backstop's tight bound.

## What would close it

Either reading is inside the band, and neither volume is wrong: the
question is which one the two orders should share. Shapes to weigh:

- normalise `d` and `ρ²` by the sphere's radius in the fan, so a
  standoff enters the fan as it enters Gauss–Bonnet, and keep the
  cancellation-free `1 − d` form for an on-sphere circle;
- or let the backstop's tight-bound reading forgive the declared pair's
  own band locally, which is `a-settled-declared-coincidence-crosses-a-tight-volume-bound`'s
  question.

The row above pins the per-order outcome at ε = 1e-12 and turns red
when either order moves.
