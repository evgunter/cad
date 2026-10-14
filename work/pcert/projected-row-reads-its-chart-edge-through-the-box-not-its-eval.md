---
id: projected-row-reads-its-chart-edge-through-the-box-not-its-eval
kind: issue
title: topo::pcurves::chart_edge reads a projected row through chart_box, because Pcurve::eval over a wide t encloses the image too loosely for far-cell membership; why is not established
status: closed
opened: 2026-10-08
priority: P3
cost: M
refs: [4304]
closed: 2026-10-14
---


Filed by the fix pass of PR 4304 (review item 13). The deviation was
disclosed in the PR body but had no row.

## What the code does

`topo::pcurves::chart_edge` gives a `Pcurve::Projected` row its
`ChartEdge::Envelope` image as the hull of `Pcurve::chart_box` over the
walked interval, with slack 0 (the `Pcurve::Projected(_)` arm, beside
the `Fitted`/`General` arm). The closed-form rows read `Pcurve::eval` at
the interval scalar's `t` enclosure instead (the natural extension).

## Why it deviates, as far as was measured

With the natural extension (`eval` at a wide `Interval` `t`), the far
cell of `crates/topo/tests/trim_3_chart_bound.rs` row t9 read the cell as
outside. Reading the box restored it. Why the extension's enclosure
misses the far cell was not investigated: it may be the `atan2`
branch's enclosure over a wide piece, or `eval`'s per-span hull of
several pieces.

## What a fix owes

Either the reason `eval`'s natural extension over a wide `t` is the
wrong reading for a projected row, stated at `chart_edge`, or an `eval`
whose enclosure is tight enough that the arm can read it as the others
do.

## Closed 2026-10-14: the reason, stated and guarded

The box read is right, and the reason now sits at `chart_edge`'s
`Pcurve::Projected` arm (`topo/src/pcurves.rs`). A spline's interval
evaluation over a wide `t` loses the correlation between its rational
quotient's numerator and denominator. The `atan2` branch is not
involved: T9 is a plane chart.

Measured on T9's shape, a degree-1 non-rational chord on a plane chart:

| `t` | `eval` (u / v) | true image |
|---|---|---|
| `[0, 1]`, the walked edge | poison / poison: the denominator `(1 − t) + t` encloses to `[0, 2]` | u = 0.5, v ∈ [−0.5, 0.5] |
| `[0, 0.5]` | [0.167, 1.5] / [−1, 0] | 0.5 / [−0.5, 0] |
| `[0.1, 0.9]` | [0.056, 4.5] / [−2, 2] | 0.5 / [−0.4, 0.4] |
| `[0.49, 0.51]` | [0.48, 0.52] / [−0.01, 0.01] | 0.5 / [−0.01, 0.01] |

`chart_box`, the per-piece control hull, is exact (u = [0.5, 0.5],
v = [−0.5, 0.5]). With the projected arm reading `eval`, T9's far cell
stops certifying outside.

**Rows.**
- `trim_3_chart_bound::t9b_a_projected_rows_eval_over_its_edge_is_poison_and_its_box_is_exact`
  guards the deviation. It goes red the day `eval`'s spline arm becomes
  tight over a wide `t`, which is when the projected arm can read `eval`
  like the closed forms.
- T9 guards the verdict: it goes red with the arm switched to `eval`.

The cause is filed for KNOT as
`work/knot/nurbs-curve-interval-eval-loses-the-quotients-correlation-over-a-wide-t.md`.
