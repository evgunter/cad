---
id: planar-ring-lane-closes-its-island-with-a-straight-chord
kind: issue
title: The planar ring lane closes a run with a straight chord even where its section is an arc: a D-shaped prism through a slab desyncs on a zero-area winding, a crescent prism on a section vertex
status: open
opened: 2026-10-02
---


## What

The planar ring lane (`boolean::join::ring_run_ccw`,
`crates/topo/src/boolean/join.rs`, through
`Body::planar_run_winding_decided`,
`crates/topo/src/loop_winding.rs:251`) winds the island run `h1 → h2`
closed by a STRAIGHT chord from `h2`'s end to `h1`'s start. Where the
section the chord lies on is an arc — a plane face pierced by a body
whose wall meets it in a circle — the straight chord is not the
island's boundary, and the winding is taken of the wrong region.

Measured by the dual review of PR 3851 (TANG), on base and head alike:

- a **D-shaped prism** (an arc plus its diameter, extruded) driven
  through a slab refuses `JoinDesync "ring-run winding is degenerate
  (zero enclosed area)"` for every positive bulge (72 of 72): the run
  is the arc's chord and the straight closure retraces it;
- a **crescent prism** refuses on base and head with `JoinDesync "a
  section vertex has no null-edge copy"` and `ZipCorrespondence "seam
  cycles differ in length"`.

Each is a kernel-class refusal of simple valid input.

## The model

The wall lane's island closure (`chord_join::chart_island_winding`,
`crates/topo/src/chord_join.rs`) integrates the closing chord EXACTLY
along the section (the plane's section of the face, in closed form).
The planar lane should close along the section conic the same way: a
circle's `∮` is the loop-area form `loop_vector_area` already
integrates exactly for a circle carrier.
