---
id: a-center-arc-near-a-full-turn-loses-its-end-vertex
kind: issue
title: paths: an arc about an authored centre nearly a full turn reads its quarter-tangent through radius + apothem, which cancels; the stored arc misses its end vertex silently at f64
status: closed
closed: 2026-10-19
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

## Outcome

Fixed in #5403, in the one home every construction reads its arc's
quarter-tangent from, `sugar::split_half_tan(across, along, len)`: the
select (`select_le_zero` on `along`) between `across/(len + along)` and
`(len − along)/across`. `quarter_tan_about` reads it on the centre's
split `(σ·h, σ·p)` of hypotenuse `r` (`sugar::centre_split`), so past a
half turn, where `σ·p` is negative, it takes `σ·(r − σ·p)/h`, which
does not cancel. The sweep stays `4·atan(X)` of a quantity algebraic in
the authored data, as the README's profile-format clause (D1) rules.
`path::center_sweep` (the `Center` leg, `family.rs`'s `Center`, a fillet
side's run, the extended arc) and `bulge_from_center` read it. The
arc-carrier fillet keeps the first arm alone: it sweeps under a half
turn, and a select is one atom to the symbolic tier's plain walk.

The row's three radii now land within 8 ulps of R at f64, where they
missed by 2.5e-8, 5.0e-6 and 2.1e-4 and validated, and their Interval
sweeps stay under 1e-13 wide. They are pinned by
`tests/tangent_arc_near_a_full_turn.rs`,
`the_centre_arc_near_a_full_turn_keeps_its_end_vertex`. At 1e-12 the
Interval lane refuses the 1.36e4 arc's equidistance, which is past
that row's scene resolution, and the row asserts that reason.
`bulge_from_center`'s relative error near a full turn is at most 1.92
units of 2⁻⁵² against the exact value of its arm (the mpmath oracle's
`C` rows, scales to 1.37e7), so the carrier it implies moves by under
2 units of 2⁻⁵²·R; through `atan2` it moved by up to 6.6 cm at
R = 1.37e7.

(The first fix pass read this sweep as an `atan2` angle between the
radii. That is an angle difference, which D1 rules out, and it cost
the pad's symbolic tier 56 decisions; it was reverted.)
