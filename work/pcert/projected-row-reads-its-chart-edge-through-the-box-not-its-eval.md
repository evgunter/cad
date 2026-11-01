---
id: projected-row-reads-its-chart-edge-through-the-box-not-its-eval
kind: issue
title: topo::pcurves::chart_edge reads a projected row through chart_box, because Pcurve::eval over a wide t encloses the image too loosely for far-cell membership
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
evaluation over a wide `t` encloses each basis function on its own, so
its sums lose the partition of unity. The `atan2` branch is not
involved: T9 is a plane chart.

Measured on T9's shape, a degree-1 non-rational chord on a plane chart:

| `t` | `eval` (u / v) | true image |
|---|---|---|
| `[0, 1]`, the walked edge | the entire line, decorated `Trv`, in both channels: the denominator `(1 − t) + t` encloses `[0, 2]` | u = 0.5, v ∈ [−0.5, 0.5] |
| `[0, 0.5]` | [0.167, 1.5] / [−1, 0] | 0.5 / [−0.5, 0] |
| `[0.1, 0.9]` | [0.056, 4.5] / [−2, 2] | 0.5 / [−0.4, 0.4] |
| `[0.49, 0.51]` | [0.48, 0.52] / [−0.0102, 0.0102] | 0.5 / [−0.01, 0.01] |

The whole-edge value is unbounded, not NaI: `Interval::is_poison` is
false for it, and its numerator is finite.

`chart_box`, the per-piece control hull, is the image's exact hull when
the walked interval is the whole net: u = [0.5, 0.5], v = [−0.5, 0.5].
Over a strict sub-range it reads the whole pieces the sub-range meets:
over `[0.1, 0.9]` it is still v = [−0.5, 0.5], against the true
[−0.4, 0.4]. That is looser than the image, but still 4× tighter than
`eval` there. With the projected arm reading `eval`, T9's far cell stops
certifying outside.

**Rows.**
- `trim_3_chart_bound::t9b_a_projected_rows_eval_over_its_edge_is_entire_and_its_box_is_exact`
  pins the cause (the entire line at `Trv` over the edge), eval's
  looseness inside it, and the box's exactness and its sub-range scope.
  A red there means re-measure and decide, not retire the box: a half
  fix that skips the division turns the whole-edge reading finite while
  `u` still reads [0, 1], where the box reads 0.5.
- `trim_3_chart_bound::t9c_a_projected_edge_on_part_of_its_net_reads_the_tighter_box`
  covers a projected edge on `[1/16, 15/16]` of its net. There `eval`
  is finite but reads [−3.5, 4.5] edge lengths along the chord, and the
  box reads [−1/14, 15/14]. A cell past the edge's far end certifies
  outside through the box.
- T9 guards the verdict: it goes red with the arm switched to `eval`.

The cause is filed for KNOT as
`work/nurbs/nurbs-curve-interval-eval-loses-the-partition-of-unity-over-a-wide-t.md`.
It has two contributors: the sums losing the partition of unity, and
`located_walk` evaluating each span past its knots.
