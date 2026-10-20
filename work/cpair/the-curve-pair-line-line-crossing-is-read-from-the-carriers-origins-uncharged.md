---
id: the-curve-pair-line-line-crossing-is-read-from-the-carriers-origins-uncharged
kind: issue
title: curve_pair's line_line places a crossing by the closed form from the two carriers' origins, with no rounding charge: its f64 error is about |o2 − o1|·u / sin φ
status: open
opened: 2026-10-18
priority: P3
cost: E
---


Found by RESTREAD's sweep for closed forms read from an arbitrary
origin (the branch `restread/no-shared-key-crossing`, which fixes the
census's own instance in `crates/topo/src/census.rs` `ee_lines_closest`).

## What

`crates/geom-brep/src/curve_pair/pose.rs` `line_line`, its `crossing`
closure: `along = (o2 − o1).cross(u2).dot(n) / s²`, read from the two
line carriers' origins `o1`, `o2`, which can stand anywhere on their
lines, metres from either span. `n = u1 × u2` cancels to an absolute
error of about `u`, and the reach's `f64` rounding is about
`(|o2 − o1| + |along|)·u / s`. RESTREAD measured the census's copy of
this form: at ε = 1e-12, 1 456 of 100 000 true crossings (read in both
orders) placed a definite span outside one edge. Nothing here charges
that rounding before `ta`/`tb` are read against the spans.

## Not traced

Whether `polish.rs` re-meters a generic candidate's place before a
verdict reads it, which would make this only a seed. The census's fix
reads each reach from the ends nearest it and charges
`geom_brep::rounding_charge((|e| + |reach| + gap/s)/s)`, deciding both
ends of the enclosure; the same shape would serve here.

## Measured: no wrong side, needless escalations (2026-10-20, NURBS build A)

Traced: the candidate is not read by its `ta`/`tb`. `Found::read`
hands its point to `Span::read`, which finds the point's foot on each
carrier again (`Curve3::foot`) and decides carrier membership
(`curve_carrier_gap`) and each end's side (`curve_pair_span_end_side`,
`_chord`) against the band. So the misplacement never decides a
side; it is read, and refused where it lands within the band's
escalation.

The error law is steeper than the title's. `n = u₁ × u₂` cancels to
an absolute error of about `u`, which the `/s²` turns into a place
along `a` off by about `|o₂ − o₁|·u/s²`, and off `b`'s carrier by about
`|o₂ − o₁|·u/s`.

**The scan** (on `main` `1c7ce8e6f0`), `f64` at `ε = 1e-12`, 40 000
reads:
- **Draws:** two lines in `z = 0` crossing at a drawn `X`, so the
  crossing is exact, at `s` log-uniform in `1e-5 … 1e-1`. The carrier
  origins lie `D` from `X` along each line, `D` log-uniform in
  `1 … 10⁴` m. `b`'s span ends `g` inside or outside the exact
  crossing (exact rationals, `exact_q::Q`), `g` log-uniform in
  `1e-11 … 1e-5`. `a`'s span runs ±1 about `X`, and both orders are
  read.
- **A wrong answer** is a reported meeting more than `e = 1e-11` from
  either span segment, or no meeting where the crossing lies inside
  both spans.

| outcome | reads |
|---|---|
| right | 25 732 |
| wrong | **0** |
| escalated `curve_carrier_gap`, predicted miss off `b` `> z` | 5 217 |
| escalated `curve_carrier_gap`, predicted miss `≤ z` | 1 827 |
| escalated `curve_pair_meet_stretch` | 5 416 |
| escalated `curve_pair_span_end_chord` / `_side` | 272 |
| precondition miss (the exact crossing within `g/2` of the end) | 1 536 |

So this is a refusal cost, not a silent error: priority P3, from P2.
- **What it costs.** The `curve_carrier_gap` escalations track the
  predicted miss, 5 217 against 1 827. They are the candidate landing
  within `b`'s band because it was placed from far origins.
- **Fixing it.** A crossing placed from the span ends nearest it (the
  census's `ee_lines_closest` shape) would decide most of them. A row
  for such a fix should count those escalations away.
- **Not this item's.** The stretch escalations are the shallow
  crossing's own: the curves are within the band of each other over
  about `z/s`.

