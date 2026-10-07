---
id: interval-axis-plane-cut-along-a-cylinder-refuses-its-rims-tie
kind: issue
title: plane_section at Interval refuses OrderEscalated (split_join_order_u) on an axis-plane cut along a cylinder's axis: the two rims' crossings share y in truth
status: open
opened: 2026-10-07
priority: P3
cost: M
---


The unit cylinder `x² + y² ≤ 1`, `0 ≤ z ≤ 1`, cut by the axis plane
`x = 0.3`, answers at `f64` (the `2√0.91` rectangle) and refuses at
`Interval`: `SectionError::Split(Join(OrderEscalated))` on
`split_join_order_u`, enclosure `±2.66e-15`. Pinned by
`a_cut_along_a_cylinders_axis_refuses_its_rims_tie_at_interval`
(`crates/sweep/tests/split_section_rings.rs`).

Cause, measured: an axis plane's join-order frame is the exact
coordinate pair (`splitting::order::in_plane_frame`, here `u = y`,
`v = z`). The plane crosses the bottom rim at `(0.3, ±√0.91, 0)` and
the top rim at `(0.3, ±√0.91, 1)`; a bottom crossing and the top one
above it are 1 apart in `v` and share `y` in truth, each computed
(a square root) to an enclosure, so `split_join_order_u` straddles.
The refusal text reads "the order of two section points is too close
to call", which is true of the order and says nothing of the points
being far apart.

Why the oblique frame that fixed the tilted cuts
(`interval-steep-cut-through-cylinder-caps-refuses-order-escalated`)
is not applied here: on an axis plane the coordinate keys are exact
for exact data, which is what lets several null edges at one point
(bit-identical copies) tie Zero at `Interval`. An oblique key
`w·d` rounds for data that is not dyadic at a few bits, so those
copies would straddle instead — trading this refusal for that one.
A fix wants both: keys separating computed crossings that tie in a
coordinate, and an exact tie for copies of one point (a topological
identity — the same vertex, or the same point record — rather than a
bit compare, which the bit-identity channel fence forbids).
