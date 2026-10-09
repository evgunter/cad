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
`MappedCurve::restrict` onto a `SweepRange`: the placement is kept as
built and the angle range is narrowed. That removed the per-split
stored rotation. Splits that keep an exact end of the range no longer
grow at all (pinned by `crates/geom-brep/tests/revolved_point_anchor.rs`
`end_anchored_splits_do_not_grow_the_stored_width`). **Interior** splits
still grow, through a different mechanism.

**The mechanism.** Each interior split rounds the composed angles
(`SweepRange::restrict`, a convex combination) by a few ulps of the
angle. `MappedCurve::eval` (`crates/geom-brep/src/mapped.rs`) turns the
point through `Affine3::rotation_about_axis(q, n, θ).transform_point(p)`,
that is `R·p + (I − R)·q`. At `T = Interval` an angle width `w` therefore
reaches the point as about `w·(|p| + |q|)`, the coordinates' magnitude,
not as `w·|p − q|`, the radius about the axis.

Measured (Interval; widest of `eval` at `s = 0, ½, 1`; a 1 m-radius rim
on a `+z` axis, nested `restrict` 64 times; N = 1 / N = 64):

| placement | split | composed placement (before) | angle range + anchored eval (now) | angle range + `p − (I − R)(p − q)` |
|---|---|---|---|---|
| near origin | (0.3, 0.7) | 3.9e-14 / 5.5e-13 | 2.2e-14 / 5.5e-13 | 4.7e-15 / 1.4e-13 |
| (1000, −700, 300) | (0.3, 0.7) | 2.2e-11 / 1.1e-10 | 1.2e-11 / 2.8e-10 | 2.3e-13 / 4.5e-13 |
| (1000, −700, 300) | (0, ½) | 7.5e-12 / 6.7e-11 | 6.3e-12 / 1.0e-12 | 2.3e-13 / 2.3e-13 |

So at a far placement, interior splits beyond about ten deep are wider
now than the composed placement was. The radius-levered spelling holds
them at the coordinates' last ulp.

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
