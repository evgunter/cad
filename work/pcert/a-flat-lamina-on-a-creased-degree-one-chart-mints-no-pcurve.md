---
id: a-flat-lamina-on-a-creased-degree-one-chart-mints-no-pcurve
kind: issue
title: The mint refuses every edge of a lamina lying wholly in the flat piece of a creased degree-1 spline chart, ResidualExceeded Envelope at sample 0
status: open
opened: 2026-11-25
priority: P2
cost: M
---



## Finding

Found by the review of PR 7787 (lane `embedwalk-trimmed`), building a
folded trimmed spline fixture for the injectivity engine; reproduced in
the lane.

The chart: degree 1 in `u` over `[0, 2]`, knots `[0, 0, 1, 2, 2]`, flat
on `u ∈ [0, 1]` (control points `(0, 0, 0)`, `(0, 1, 0)`, `(1, 0, 0)`,
`(1, 1, 0)`), creased at `u = 1` and rising to `z = 1` at `u = 2`;
degree 1 in `v` over `[0, 1]`. The lamina `[0, 0.5] × [0, 1]` lies wholly
in the flat first piece. It is built as `clearance_windows_on_a_spline_chart.rs`'s
`lamina_in` builds one: `mvfs`, `mev_line`s, a `mef_chord`.

`topo::mint_pcurves_of` refuses:

`Err(Certify { half_edge: HalfEdgeKey(1v1), error: ResidualExceeded { check: Envelope, sample: 0 } })`

That is the first edge, a straight line on a flat chart piece, whose
image is exact. The reviewer reports the same refusal for every creased
degree-1 chart that is not flat. So no test can build a trimmed spline
face on a chart that folds outside its trim, which is the case where
the injectivity engine's monotone arm reads more than the face.

## What is needed

The envelope check should read the image against the chart piece it
lies on, or say why a crease anywhere in the chart enters an edge that
lies off it.
