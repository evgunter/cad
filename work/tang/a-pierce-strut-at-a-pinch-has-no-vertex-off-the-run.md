---
id: a-pierce-strut-at-a-pinch-has-no-vertex-off-the-run
kind: issue
title: Holes touching at a corner refuse RingHomingAmbiguous when the blocks fold first: the pinch's strut ring has no vertex off the run
status: open
opened: 2026-10-02
priority: P1
cost: M
---

## What

Two blocks whose footprints are holes in a plate's top touching at one
corner. The plate is `[0,3] × [0,2] × [0,1]`,
`p1 = block((1.0,1.5),(0.5,1.0),0.5,1.5)` and
`p2 = block((1.5,2.0),(1.0,1.5),0.47,1.23)`, with the `block` helper of
`crates/editor-core/tests/docm7_union_declare.rs`. The union builds
F16 E37 V24 at volume 6.425 in four member orders, and refuses
`Boolean(Join(RingHomingAmbiguous))` in `[p1, p2, plate]` and
`[p2, p1, plate]`. The notch-plus-hole variant (`p1` with y ∈ (−1, 1))
builds F17 E43 V28 at 7.425 in five orders and refuses the same way in
`[p2, p1, plate]`. Measured on `tang/pinch-union-order`, 2026-10-02.

## Cause

When the blocks fold first, their contact edges pierce the top at the
pinch (1.5, 1, 1) once each, and `vtxfac` mints one ring per pierce. The
first section polygon's join divides the top. The run is the
polygon's own outline, through the first pierce's vertex, and
`ChordJoiner::rehome_rings` (`crates/topo/src/chord_join.rs`) then homes
the second pierce's ring. That ring is a strut: one zero-length null
edge whose two vertices both sit at the pinch. An instrumented run
dumped it as `[23v1 (1.5,1,1)] [24v1 (1.5,1,1)]`, one edge, against
the run `(1.5,1) (1,1) (1,0.5) (1.5,0.5)`. So every vertex lands
`OnBoundary`, and `ring_side`'s rule (the first vertex off the run)
has nothing to read.

No rule that reads the ring's vertices, or any point of it, can decide
this. The datum that does is the direction the strut's own section
polygon leaves the pinch: the two germs of its null edge
(`BoolNullEdgeRecord::germs`, `HalfGerm::dir`), oriented by the face's
normal so the polygon's side of the germ pair is known. The germs live
in the boolean's null-edge records, and `ChordJoiner` is side-agnostic
and shared with the split sweep, so they never reach `rehome_rings`.

## Candidate mechanisms

- Carry the strut's germ wedge into the homing through `JoinLane`.
  The ring is `In` when the wedge lies inside the run's wedge at the
  pinch. This needs the polygon's interior side at the point, not only
  the bisector of the two germs, because an interior angle above π flips
  the bisector.
- Defer the strut: leave it unplaced at the split and home it with the
  rings its polygon will be joined to. Those were homed by their own
  vertices, off the run.

Either is a change to the join's contract, not to the homing test, so it
was left out of the PR that fixed the coincident-pierce weld.
