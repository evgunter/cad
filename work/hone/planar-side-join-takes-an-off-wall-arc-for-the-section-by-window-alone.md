---
id: planar-side-join-takes-an-off-wall-arc-for-the-section-by-window-alone
kind: issue
title: The planar-side join takes a conic between edge for the section segment by window membership alone, never asking whether the arc lies on the wall
status: closed
opened: 2026-10-01
closed: 2026-10-02
cost: E
priority: P3
---

(REACH, PR 3627's delta review: the sibling of the line defect that PR
fixed, on the same function.)

## Closed as moot (JOIN-1, PR 3790)

The arm is gone. The boolean lanes' adjacency skip no longer asks the
between edge's geometry: it fires only when the between edge IS the edge
the matched germs' locus names on that solid (`chord_join::Chords::Segment`),
and a segment inside a face names none, so a foreign arc can never be
taken for the section. `between_edge_is_section` answers the split lane
alone: it takes the split's `SectionCtx`, so a boolean lane cannot ask
it (JOIN-3). The text below is the record of the defect as it stood.

## What

`chord_join.rs` `between_edge_is_section` (~1694) answers the join's
adjacency skip: is the edge between two adjacent ON copies already the
section segment, so no chord is minted? In the boolean planar-side lane
(`JoinLane::BoolPlanar`, a planar face divided by a cylinder or sphere
germ) its two carriers are asked different questions:

- a LINE asks whether it lies ON the wall (`bool_between_line_on_wall`,
  its midpoint's `BoolWall::off_wall`), which PR 3627 added after the
  structural `true` skipped the section arc beside a wall chord and made
  wrong ∩ bodies;
- a CIRCLE or ELLIPSE (the arm at ~1766, `bool_between_arc_window`)
  asks only whether its midpoint's azimuth lies in the wall face's
  window. It never asks whether the arc lies on the wall.

A conic edge of the planar face whose two ends are on the wall and whose
midpoint's azimuth falls in the window, but which is NOT the wall's
section (another surface's circle — a second drum's cap rim, or a fillet
arc in the plane), would read `Some(true)`: the chord is skipped, the
section arc beside it is never minted, and the zip glues the wrong pair.
That is the shape PR 3627's fix closed for lines.

## Why it is unreached today

The two ON copies on such an arc are where the arc crosses the wall:
a circle against a cylinder or sphere that is not its own section. With
parallel axes that is the rim-circle-against-a-wall crossing, whose
parameters are roots of a degree-2 trigonometric polynomial with no root
lane (`verbs_germarms2::the_fenced_poses_keep_their_own_doors`,
parallel-equal-r: `CurvedPierceUnsupported`). The curved×curved crossing
is refused upstream, so no op hands this arm such an arc.

## What would reach it, and the fix

The day a circle×wall root lane lands (parallel drums of different radii
overlapping, a planar face carrying one's cap rim crossing the other's
wall), the arm should also ask the arc's midpoint `off_wall` (the line
arm's test) and mint the chord when it is definitely off the wall:
window membership picks which of the section's two arcs is this side's,
not whether a foreign arc is the section at all. A row with a second
drum's rim arc inside the first's window, asserting closed-form volumes
and tier 3, pins it.

