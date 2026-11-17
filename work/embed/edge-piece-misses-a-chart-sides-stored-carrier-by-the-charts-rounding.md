---
id: edge-piece-misses-a-chart-sides-stored-carrier-by-the-charts-rounding
kind: issue
title: The graph's edge piece is read off the presentations' image boxes, which can miss the stored carrier's points by the chart's rounding, so a spline side under the graph is read on its edge up to that rounding
status: open
opened: 2026-11-17
priority: P2
cost: M
refs: [the-graph-separator-accepts-a-side-past-its-edges-piece-by-membership]
---


## Finding

Found by embed/graph-piece (PR 7080), measuring.

`graph_of` (`crates/topo/src/injective.rs`) builds the graph's piece with
`edge_piece`: the hull of the edge's parameters whose arc enclosure meets
either presentation's image box (`Presented::whole`). The box is the
chart's rectangle enclosure, and it can exclude the stored carrier's own
points by a rounding.

**Measured** on sweep's `a_filleted_drum_certifies_across_its_fillet`: a
cylinder face's rectangle `u ∈ [π/2, π]`, `v ∈ [0, 1.7]` has image box
`y ∈ [1.2246e-16, 1]`, from `sin π` rounded. The shared ruling at `u = π`
is stored at `y = 0`, so its points miss that box, and the piece the edge
reaches is `(0, 0.85)` while the side on it runs over `(0, 1.7)`.

**What PR 7080 does about it.** A side whose chart line is its edge's own
line or circle has its parameters read and folded into the graph's piece
(`graph_of`), so the graph vanishes on all of it. A spline side
(`Along::whole` false) has no parameter map, and `sign_on` reads it zero
under the graph where it lies on its edge (`Along::on_edges`), as main
read it. Its points are the edge's, but whether the piece holds them is
the rounding above, unbounded by any check. Such sides reach the graph
in sweep's suites (4,866 readings, the lofts' walls among them).

## What is needed

Read the piece so that it provably holds every point of a side on the
edge. One way: build the boxes `edge_piece` meets from the stored
carrier's own enclosure over the side's span, not from the chart's.
Another: give a spline side parameters by the edge's own pcurve. Measure
first whether any spline side's points fall outside the piece.
