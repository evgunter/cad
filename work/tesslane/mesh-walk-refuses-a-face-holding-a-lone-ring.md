---
id: mesh-walk-refuses-a-face-holding-a-lone-ring
kind: issue
title: mesh's loop walk refuses a whole face holding a lone-vertex ring as EmptyLoop, though the ring bounds no area (topo's FaceView::area_loops reads past it)
status: open
opened: 2026-11-23
---

Found by WALKS' face-region unit (PR 7474). Topo now reads a lone-vertex
ring as a point on the boundary that bounds no area
(`crates/topo/src/live.rs` module docs, `FaceView::area_loops`), and
`Body::face_lift` lifts a face's outline and its cycle rings alone.
`crates/mesh/src/walk.rs` `loop_half_edges` still re-decides it: a loop
that is not `LoopBoundary::Cycle` refuses the whole face
`TessellateError::EmptyLoop`, so a wall holding a lone ring that
`topo::validate` passes is not meshed
(`crates/mesh/tests/lift_probes.rs`
`a_ring_that_is_one_vertex_bounds_no_area` pins today's refusal).
`trimmed.rs` `loop_polygon` refuses an empty lift the same way.

Fix: read the face's loops through `FaceView::area_loops` (or
`cycle_loops` for a half-edge question), take the cycles, and refuse
only `NoOutline`; then that probe asserts the wall meshes. The
discard register's `crates/mesh/src/walk.rs|loop_half_edges` entry goes
with it.
