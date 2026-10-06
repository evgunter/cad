---
id: f64-cannot-place-a-shallow-crossing-within-the-finest-band
kind: issue
title: At eps 1e-12 an f64 first-order bound cannot place a circle x sphere crossing of slope below ~1e-3: the near-tangent snowman stops at delta 1e-7
status: open
opened: 2026-10-02
priority: P3
cost: H
design: true
refs: [circle-sphere-root-slack-refuses-near-tangent-pairs-at-1e-12]
---


## Measured (branch `reach/circle-sphere-slack`)

The near-tangent snowman (`crates/sweep/tests/snowman.rs`: balls r 1.0
at `y = 0` and r 0.8 at `y = 1.8 − δ`), at ε 1e-12, after the circle ×
sphere root slack was derived from the near extreme's own running
rounding bound (`geom_brep::circle_sphere_harmonic`'s `lo`/`lo_error`,
read by `circle_roots::first_harmonic_roots`):

| δ | slope `√(−lo·hi)` | `lo_error` | slack (m) | outcome |
| --- | --: | --: | --: | --- |
| 1e-5 | 6.71e-3 | 1.47e-15 | 2.30e-13 | builds, all four ops |
| 1e-6 | 2.12e-3 | 1.47e-15 | 7.02e-13 | builds, all four ops |
| 1e-7 | 6.71e-4 | 1.47e-15 | 2.20e-12 | refuses (gap): `CurvedPierceUnsupported` |
| 1e-8 | 2.12e-4 | 1.47e-15 | 6.92e-12 | refuses (gap) |

`lo_error ≈ 13u` of the pose's unit length scale (`u = 2⁻⁵³`): the
chain `e = C₀ − c`, three dot products, two hypots and the factored
product, each operation charged `u·|result|`. The bound is
pessimistic, but not by much: the dual review of PR 3847 measured
`|lo − lo*| ≤ 0.58·lo_error` over 27k poses against an mpmath
evaluation of the factored form, so a bound sharpened to the measured
worst case is about 1.7× lower. That does not move the frontier: at
δ 1e-7 the slack would be `2.2e-12 / 1.7 ≈ 1.3e-12`, still past the
band. A correctly rounded evaluation of anything of size ~2 m is
already off by up to 2.2e-16, so at slope `s` the slack is of order
`lo_error / s` whatever the count, and the band 1e-12 needs
`s ≳ 1.5e-3` (`≳ 9e-4` at the measured worst case), i.e. `δ ≳ 5e-7`
(`≳ 2e-7`) on this pose. Pinned by
`the_near_tangent_family_stops_at_1e_7_at_eps_1e_12` (sweep) and
`the_root_slack_meter_refuses_a_reading_in_the_band_gap` (topo).

At ε 1e-9 the default band still builds down to δ 1e-7 and escalates
at 1e-8 on `bool_vertex_face_side`, unchanged.

## What would move it

Only an evaluation of the near extreme more accurate than f64 rounding:
a compensated chain (Knuth's 2Sum, which `geom_core::exact::two_sum`
already provides on `f64`, and an FMA-free Veltkamp/Dekker two-product)
carries `D₋ − r` to `~u·|D₋ − r| + u²·L`, and the slack becomes the
phase's and the angle arithmetic's alone. That is a **design
question**: the door is generic over `T: Real`, the scalar contract
excludes fused operations (`geom_core::real` module docs) and has no
error-free transform, and an `Interval` instantiation of one would
need its own meaning. It is not answered by a ratified clause, so it
is not chosen here.

Whether the f64 lane should reach that far at all is the other half of
the question: ε 1e-12 is a suite row, four decades under the default,
and a crossing at slope 7e-4 is placed to 1e-12 only by arithmetic
finer than the operands' own coordinates carry.
