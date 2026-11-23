---
id: a-lofted-segments-sub-arc-pieces-are-named-by-position
kind: issue
title: A lofted segment's sub-arc walls, rims and joints are named by their position along it, so an edit that changes a section's sub-arc count re-binds SubArc(q) with no strand report
status: open
opened: 2026-11-22
priority: P2
cost: M
---


Found by LATHE's `lathe/skin-sub-arc-walls` (the unit that cuts a lofted
wall at each sub-arc joint, `work/lathe/the-skin-cuts-an-arc-wall-at-each-sub-arc-joint.md`).

## Finding

The loft now cuts a segment's wall at every C⁰ knot its skinned surface
carries in `u`: the union of its sections' sub-arc joints
(`crates/sweep/src/loft.rs`, `sub_arc_walls`). The emitter names each
piece after the whole segment's name, with the piece's index along it
(`crates/editor-core/src/names/emit_sweep.rs`, `name_loft_topology`;
`names/README.md`, "Loft walls"): `[LoftWall(..), SubArc(q)]` for a wall,
`[RimEdge(..), SubArc(q)]` and `[LoftSectionEdge(..), SubArc(q)]` for its
edges, and `SubArcJoint(q)` after the same heads for a joint.

The index is a position. Where the joints move, the names stay spelled
the same and denote other entities. A value edit can change a section's
arc sweep across a multiple of the sub-arc ceiling, about 95.6°
(`sweep::skin::sub_arc_count`). A section can also be added whose arc
takes a different sub-arc count. Either way the wall's joints move: a
half turn's two quarters, `SubArc(0)` and `SubArc(1)`, become three
pieces of 66.7°, and `SubArc(1)` now names the middle third. Nothing
reports the change, because the name still resolves. This is the silent
re-binding the loft's section ids were minted to stop
(`work/refs/loft-slab-and-section-names-are-positional.md`), one level
down. `a-loft-reshape-reports-no-stranded-name` reports names that
vanish. It does not report names that re-bind.

## What is owed

A decision on what a sub-arc piece is named by, so that an edit that
moves the joints either keeps each name on what it named or strands it
loudly. One candidate is to name a piece by the sub-arc counts that cut
the segment, so a different count is a different name and the old one
vanishes into the strand report. Another is to name it by the joints'
sections and indices. The ruling the cut follows (DESIGN [topology],
Ev on `[ev]` PR 5913) accepts that the face count follows the ceiling.
It does not say how the names follow it.
