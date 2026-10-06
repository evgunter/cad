---
id: tangent-witness-span-reads-vertices-not-arcs
kind: issue
title: the tangent declaration's witness_span projects a face's boundary VERTICES only, so an arc-bounded face's span along the locus is under-read
status: open
opened: 2026-10-06
priority: P2
cost: E
refs: [an-edges-extent-has-four-dispatchers]
---


Found by reading, during the `shell/planar-gate-misses` lane's sweep for
extents folded from boundary vertices (the shape of
`shell-clearance-footprint-reads-vertices-not-arcs`). Not witnessed by
execution.

`witness_span` (`crates/topo/src/boolean/mod.rs`) projects the two
declared faces' boundary VERTICES (`face_boundary_points`, the
`boolean/mod.rs` copy) onto the locus line and returns their union — the
extent a tangent witness is metered over — and their overlap, where the
witness samples. An arc-bounded face reaches past its vertices along the
line (an extruded disc's cap: two vertices on one diameter, so a line
across that diameter reads a span of zero), so the union under-claims
the metering extent and the overlap can miss where the faces meet.

A fix folds each curved edge's extent in through the one door
`an-edges-extent-has-four-dispatchers` names. Until that lands it is
`splitting::containment::carrier_ball`, which the shell clearance gate
reads (`shell::carrier_box`).
