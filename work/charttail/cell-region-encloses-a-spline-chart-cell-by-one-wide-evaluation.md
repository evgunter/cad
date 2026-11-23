---
id: cell-region-encloses-a-spline-chart-cell-by-one-wide-evaluation
kind: issue
title: cell_region encloses a cell of a spline chart by one interval evaluation over the cell, which is the whole line over a cell spanning a knot span
status: open
opened: 2026-11-18
priority: P3
cost: E
---

## Finding

Filed by EMBED's lane `embed/rational-window` (2026-11-18), from its
sweep for an interval evaluation of a carrier over a whole span.

`cell_region_lifted` (`crates/topo/src/cell_region.rs`) bounds a cell's
reach from its station by `surface.eval(span(u), span(v))`, one
interval evaluation over the cell, for every carrier kind. On a spline
chart that is `NurbsSurface::eval` at a wide parameter, whose basis
enclosures lose the partition of unity: over a cell that holds a whole
knot span the reach is the whole line (the curve case measured in
`work/nurbs/nurbs-curve-interval-eval-loses-the-partition-of-unity-over-a-wide-t.md`;
`work/nurbs/nurbs-interval-ders-at-a-wide-parameter-grows-with-translation.md`
is the surface `ders` face). The cell is then `Undecided`, which is
sound, and a smaller cell inside one span reads finite but several
times too wide.

**No engine caller reaches it on a spline chart today.** The clearance
sweep's windows of a spline face are `Region::Whole`, which skips
`cell_region_lifted`; the public `cell_region` is read by the tests in
`crates/sweep/tests/cell_region_bodies.rs` and
`crates/topo/tests/projected_row_chart_box.rs`, not measured here on a
spline face.

**The fix** is the surface's one reading over a rectangle,
`Surface::rect_enclosure` (`crates/geom/src/surfaces.rs`), which reads a
spline chart's restricted net and is `eval` at the two intervals for an
analytic carrier: the reach would not change on any carrier that reaches
it today.
