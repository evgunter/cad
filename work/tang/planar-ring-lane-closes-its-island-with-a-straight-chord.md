---
id: planar-ring-lane-closes-its-island-with-a-straight-chord
kind: issue
title: The planar ring lane closes a run with a straight chord even where its section is an arc: a D-shaped prism through a slab desyncs on a zero-area winding, a crescent prism on a section vertex
status: closed
opened: 2026-10-02
priority: P0
cost: M
closed: 2026-10-06
branch: tang/planar-ring-arc-closure
pr: 4133
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

## Review tier

SINGLE, FULL: the ring lane's pairing and closure decide which body a
common boolean builds; one full review each.

## Closed

Fixed upstream before this unit was dispatched, by JOIN's PR 3895
(`join/3-segment-curve`, JOIN-3): a matched section segment carries its
chord curve (`chord_join::SegmentCurve`), and the ring lane closes its
run along it (`loop_winding::RunClosing::Curve`), which is the closure
this item asked for. No kernel change rides here.

Measured on a probe of 330 ops (D, crescent, lens and split-arc
profiles, upright, off the origin, spun and tilted once and twice, all
six ops including both member orders): 150 refuse at PR 3895's parent
`4987431a^1` — every D, `JoinDesync "ring-run winding is degenerate
(zero enclosed area)"`, and the crescent, `"minted-edge description
failed certification"` — and none at its merge or on `46aad757`. The
two refusals the dual review of PR 3851 saw on the crescent (`"a
section vertex has no null-edge copy"`, `"seam cycles differ in
length"`) are not reached by these shapes at either commit.

The unit adds
`planar_ring_arc_closure::a_prism_with_arc_walls_through_a_slab_builds_every_op`:
those profiles at those poses, every op in both member orders, held to
tiers 1–3, the closed-form volume (exact upright, within the measured
pad where a tilt bounds a face by ellipse arcs) and the exact census,
derived from the prism's sides and the vertices drawn on its arcs
(PR 3826 sweeps a run of cocircular arcs as one wall, so a vertex
drawn on an arc stays on the prism's caps and leaves none in the
section). Tier 3′ passes
everywhere but on the two stubs of a tilted `prism ∖ slab`, where
CONTACT's census cannot yet decide two parts' curved faces apart; the
row pins that refusal and the CONTACT issue carries the witness. Red at
PR 3895's parent on its first row, with this item's refusal; green on
`46aad757`.
