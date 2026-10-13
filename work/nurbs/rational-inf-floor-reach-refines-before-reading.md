---
id: rational-inf-floor-reach-refines-before-reading
kind: issue
title: The rational inf floors refuse nets whose weights spread past about 1.6×; refining the cells before reading would recover reach
status: open
opened: 2026-10-13
priority: P2
cost: M
design: true
---

## What

`geom_core::spline::rational_stretch_floor` and `rational_area_floor`
(PR `nurbs/rational-inf-floor`) read each pair term over the whole
cell's hull of active points, with the cell's whole weight range. The
relaxation is loose where the weights vary inside a cell: on the
biquadratic single-cell nets of `sweep/tests/r2_probe_cert8.rs`
(`WRITTEN_CHARTS`), the inf-arm assembly enters a rational net only
up to a weight spread of about `1.4×` to `1.6×`:

| net | enters at weight spread | refuses at |
|---|---|---|
| flat-rational | `1 + 0.2·(w − 1)` | `0.3` and above (the original `7.5×`) |
| bowed-rational | `1 + 0.2·(w − 1)` | `0.3` and above |
| sheared-rational | `1 + 0.3·(w − 1)` | `0.4` and above (the original `3.7×`) |

The ratio-squared bracket this replaced entered all three, without a
proof. `probe_steep_rational_charts_refuse_at_the_pair_term_floor`
pins the frontier.

## Shape

Knot insertion shrinks each cell's hull and its weight range, and the
pair-term floor of the refined net is still a floor of the same
surface, so reading the floor after inserting knots (to a fixed depth,
or until each cell's weight spread is under a bound) converges toward
the true inf at a cost in cells. Whether that cost belongs in
`chart_stretch_inf`, which `topo`'s `certified_arms` calls per face
pair, is the question; the curve meter's piece assembly
(`rational-inf-speed-has-two-recipes-curve-and-surface`) is the other
data point on reach.
