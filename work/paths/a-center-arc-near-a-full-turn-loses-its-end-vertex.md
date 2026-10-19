---
id: a-center-arc-near-a-full-turn-loses-its-end-vertex
kind: issue
title: paths: an arc about an authored centre nearly a full turn reads its quarter-tangent through radius + apothem, which cancels; the stored arc misses its end vertex silently at f64
status: open
opened: 2026-10-19
priority: P1
---


Found by the sweep of `tangent-arc-near-a-full-turn-loses-its-end-vertex`
(the same cancellation, in a different spelling).

**The defect.** `sugar::quarter_tan_about` spells the quarter-tangent of
an arc about a known centre as `sgn·(L/2) / (r + sgn·offset)`, the
offset being the centre's signed distance from the chord's midpoint.
Near a full turn the centre sits on the far side of a short chord, so
`sgn·offset → −r` and the denominator, whose true value is about
`r·(2π − θ)²/8`, carries a rounding of about `ulp(r)`. The stored sweep
loses its digits and the arc misses its end vertex. The callers are
`path::center_sweep` (the `Center` leg, `family.rs`'s `Center` close, a
fillet side's carrier run) and the arc-carrier fillet's bulge
(`sugar.rs`, `quarter_tan_about(t1, t2, center, radius, sgn)`); a fillet
arc sweeps under π, so only the `Center` and side-run callers reach the
near-full turn.

**Measured** (probe: `At(0,0) ArcTo(Center { c: (0, R), Ccw, b }) LineTo((-1.35, -5)) LineTo(Start)`,
`b` on the carrier 2.7 m of arc short of the full turn, default ε):

| R | f64: stored arc's miss of `b` | f64 `validate` | Interval landing width |
|---|---|---|---|
| 1.36e4 | 2.5e-8 (8.2e3 ulps of R) | **Ok, silent** | 1.0e-6 (refuses: span undecided) |
| 1.37e5 | 5.0e-6 | **Ok, silent** | 1.2e-4 |
| 1.37e6 | 2.1e-4 | **Ok, silent** | 1.0e-2 |
| 1.37e7 | refuses `path_arc_center_equidistant` (scene resolution) | | |

The `Center` arc's facts are `Decided`, and nothing re-reads the
stored sweep against the end vertex, so the miss passes validation.

**Fix shape.** The tangent arc and `Via` now choose between their two
half-angle spellings, `s/(1 + c)` and `(1 − c)/s`, with
`Real::select_le_zero` on the sign of the cosine term (the tangent-arc
row's fix). So every arc up to a half turn keeps its bits and its
symbolic forms. Do not take the sum-of-squares spelling of `1 + cos`
here: it is cancellation-free, but it rebuilt the stadium's symbolic
forms, and the link study (`m10_9_pins_interval`) stopped certifying.
This case has a third input. `r = |a − c|` is exact, but `b` is on the
carrier only to a decision. With `h` the half-chord and `t` the
centre's along-chord offset, `r + e = (h + t)²/(r − e)` is the identity
that holds for an off-carrier `b`. Selecting it on the sign of the
apothem `e` is the likely shape. It moves the stored bits of every
`Center` arc and fillet side run past a half turn.
