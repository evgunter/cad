---
id: three-wedges-meeting-at-a-point-on-a-face-refuse-in-every-member-order
kind: issue
title: Three wedges meeting at one point of a plate's top refuse in every member order, with three different refusals
status: open
opened: 2026-10-02
---


## What

The plate `[0,3] × [0,2] × [0,1]` with three triangular prisms standing on
it, z ∈ (0.5, 2.0). Their plans are the sectors 0°–60°, 120°–180° and
240°–300° of radius 0.4 about (1.5, 1.0). The prisms touch pairwise only
along the vertical line through (1.5, 1.0), and their footprints on the
plate's top are three holes meeting at one vertex. The union refuses in
all 24 member orders, and which refusal depends on the order:

- `Join(RingHomingAmbiguous)` when two prisms come before the plate
  (`crates/topo/src/chord_join.rs`, `rehome_rings`);
- `JoinDesync { "ring-run winding is degenerate (zero enclosed area)" }`
  (`crates/topo/src/boolean/join.rs`, `ring_run_ccw`);
- `ClassificationInvariant { "two distinct same-solid bounds share one
  ray (degenerate operand)" }`.

The counts were the same before and after the branch
`tang/pinch-union-order`. That branch welds two pierces that meet on one
kept face (`crates/topo/src/boolean/finish.rs`, `weld_pinches`), which
covers two holes meeting at a corner. The wedges never reach the weld:
every order refuses earlier, in the join, and which step raises each of
the three refusals was not traced. What is missing is building a face
whose holes pass through one vertex, in any order. The probe
was a scratch test over these members (`block` and an `on_frame` prism in
`crates/editor-core/tests/docm7_union_declare.rs`'s helpers), run in
every member order through `fixture::union_over`.
