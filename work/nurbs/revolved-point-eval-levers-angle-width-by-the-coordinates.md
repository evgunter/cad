---
id: revolved-point-eval-levers-angle-width-by-the-coordinates
kind: issue
title: RevolvedPoint::eval's anchored rotation carries an angle's interval width to the point times the coordinates' magnitude, so interior restrictions still grow at a far placement; the radius-levered spellings cost f64 agreement on origin-axis geometry
status: open
opened: 2026-10-09
refs: [mapped-curve-restrict-composes-placements-per-split]
priority: P2
cost: M
---

Found by `nurbs/restrict-in-the-parameter`, which moved
`MappedCurve::restrict` onto a `SweepRange` (a start and a stored span):
the placement is kept as built and the range is narrowed. That removed
the per-split stored rotation. Splits from the start of the range
(`(0, ½)`, `(0, a)`) no longer grow at all, and every other chain
measured is within 2× of the composed placement, most well under it
(pinned by `crates/geom-brep/tests/revolved_point_anchor.rs`
`splits_from_the_start_do_not_grow_the_stored_width`,
`interior_and_alternating_splits_stay_under_the_composed_cost` and
`restriction_is_never_much_wider_than_a_composed_placement`). What still
grows is one mechanism, and this row is about it.

**The mechanism.** A split whose start moves by an inexact amount
rounds the range's start once, at the angle's own ulp (`start +
span·s0`). `MappedCurve::eval` (`crates/geom-brep/src/mapped.rs`) turns
the point through `Affine3::rotation_about_axis(q, n, θ).transform_point(p)`,
that is `R·p + (I − R)·q`. At `T = Interval` an angle width `w` therefore
reaches the point as about `w·(|p| + |q|)`, the coordinates' magnitude,
not as `w·|p − q|`, the radius about the axis. Far from the origin that
lever turns one ulp of the angle per split into ~3e-12 per split.

A second mechanism that the PR's first heads had is gone: storing the
range as two endpoints and evaluating `from·(1 − s) + to·s` multiplied
the parameter's own width by `|from| + |to|` and never contracted, which
reached 2.8e-10 on `(0.3, 0.7)` far and 1.1e-7 on alternating `a ±
1e-13` far. The start-and-span form adds only `|span|·width(s0)` and one
rounding per split.

Measured (Interval; widest of `eval` at `s = 0, ½, 1`; a 1 m-radius rim
on a `+z` axis, nested `restrict` 64 times; N = 1 / N = 64):

| placement | split | composed placement (main) | start + span, anchored eval (PR) |
|---|---|---|---|
| near origin | (0.3, 0.7) | 3.9e-14 / 5.5e-13 | 3.2e-14 / 2.4e-13 |
| near origin | (½, 1) | 1.8e-14 / 3.0e-13 | 1.0e-14 / 4.4e-13 |
| (1000, −700, 300) | (0.3, 0.7) | 2.2e-11 / 1.1e-10 | 1.8e-11 / 1.2e-10 |
| (1000, −700, 300) | (½, 1) | 1.0e-11 / 1.2e-10 | 6.1e-12 / 2.2e-10 |
| (1000, −700, 300) | (0, ½) | 7.5e-12 / 6.7e-11 | 6.3e-12 / 1.0e-12 |

A flat floor, measured by composing the chain at `f64` and evaluating
once at an ulp-wide enclosure of the result, is ~1e-11 far and ~2e-14
near at every N. The gap is the per-split rounding times the lever.

The radius-levered spelling `p − (I − R)(p − q)` (measured on the PR's
endpoint-form head) held the far `(0.3, 0.7)` chain at 4.5e-13 and far
`(0, ½)` at 2.3e-13, the coordinates' last ulp.

**Why the respell is not simply taken.** Two spellings carry the angle
on the offset `p − q`:

- `p − (I − R)·(p − q)` (`Mat3::identity_minus_rotation_about`) keeps
  the start-sample anchor property that
  `the_revolved_anchor_contributes_no_width_at_the_start_sample` pins.
- `p − 2·sin(θ/2)·(sin(θ/2)·v⊥ − cos(θ/2)·(n × v))` (Rodrigues anchored
  at `p`, `v = p − q`) is the other.

At `f64`, on an axis through the origin with a large radius, both agree
with the carrier's own evaluation worse than `R·p` does. On
`crates/topo/tests/mesh8_coherence.rs` `tilted_lune` (R ≈ 2.3e6 m,
ε = 1e-9, the radius chosen to sit at the band), the max
`|description − carrier|` over 17 samples per edge is:

| spelling | range over 10 edges |
|---|---|
| `R·p + (I − R)·q` (shipped) | 4.7e-10 – 7.0e-10 |
| `p − (I − R)·(p − q)` | 5.7e-10 – 1.9e-9 |
| Rodrigues at `p` | 6.7e-10 – 1.05e-9 |

With the `(I − R)` spelling, that row's `the fixture must build` went
red on hosted CI (PR 4441's first two heads), though it stayed green
locally. At far placements the same spelling moved
`crates/sweep/tests/sym11_far_placement_rows.rs`: the washer stopped
refusing on `MappedSource` and reached `Surface2Residual`, or built.

So the choice trades `f64` agreement on origin-axis, large-radius
geometry (about 2× worse) for interval width and `f64` accuracy at far
placements (orders better). It is a design call on how the description
should be evaluated, not a mechanical fix. The data above is the input
for that call.
