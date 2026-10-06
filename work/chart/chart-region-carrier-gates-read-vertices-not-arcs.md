---
id: chart-region-carrier-gates-read-vertices-not-arcs
kind: issue
title: chart_region's carrier gates (plane tilt worst, cylinder axial reach) read a face's boundary VERTICES only, so an arc-bounded face's tilt or reach is under-read
status: open
opened: 2026-10-06
priority: P2
cost: E
---


Found by reading, during the `shell/planar-gate-misses` lane's sweep for
extents folded from boundary vertices (the shape of
`shell-clearance-footprint-reads-vertices-not-arcs`). Not witnessed by
execution.

`crates/topo/src/chart_region.rs`'s `face_boundary_points` returns each
half-edge's START vertex and nothing on an edge's interior, and two
carrier gates fold it as if it were the face's extent:

- `carrier_agreement` (`chart_region_carrier_tilt`): the worst distance
  of either face's boundary from both plane descriptions. Two planes
  tilted about a line through the vertices read `worst = 0` at every
  vertex while an arc-bounded face's far side stands `sin θ · r` off
  the other plane: an extruded disc's cap has its two vertices on one
  diameter, so a tilt about that diameter is invisible.
- the cylinder arm's `reach` (the axial reach that builds the transfer
  lever `hyp`): an edge whose axial coordinate is not monotone between
  its vertices (an ellipse from an oblique cut, a spline) reaches past
  them, so the lever is under-claimed — the permissive direction for a
  gate that decides `Zero`.

`transfer_residual`, the third reader, pairs vertices positionally by
design and is not this class.

A fix folds each curved edge's extent in (`containment::carrier_ball`,
or `geom::curves::boxes::conic_arc_aabb` on the edge's carrier).
