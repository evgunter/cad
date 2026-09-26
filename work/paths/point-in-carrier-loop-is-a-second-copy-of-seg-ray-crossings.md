---
id: point-in-carrier-loop-is-a-second-copy-of-seg-ray-crossings
kind: issue
title: point_in_carrier_loop's ray x arc crossing is a second copy of profile::seg's ray_crossings (planar ray parity with arcs, two spellings)
status: open
opened: 2026-09-25
priority: P2
cost: D
refs: [ATREST-9]
---

Found by the ATREST-9 dual review (PR #3204). Filed on PATHS because it
owns `crates/profile/src/seg.rs`, the first copy.

**Two implementations of one walk.** `profile::seg::ray_crossings`
(`seg.rs`, near the `ray_side`/`ray_advance` rows) counts an in-plane
ray's crossings of a profile loop's segments, arcs included: a line by
the endpoints' straddle, an arc by `carrier_line_circle` plus
`arc_span`, every tangency or endpoint hit a graze. ATREST-9 added a
second one on the B-rep side,
`topo::splitting::containment::point_in_carrier_loop` (with
`conic_crossings` and `ConicArc::in_window`): the straight edges through
`ray_parity::ray_crossings`, each circle or ELLIPSE arc as a root of
`|P + D·s|² = 1` in the arc's unit coordinates inside a cosine window.
Same question, two spellings, different row names
(`ray_side`/`ray_advance` against `point_in_arc_loop_*`), different
discriminant forms, and the B-rep copy covers ellipses where the
profile one does not.

**Sibling filing.** ATREST-11's
`check-9-arm-5-is-a-second-copy-of-seg-planar-edge-pair-intersector`
(on its branch at filing time) records the same shape for the
edge-pair intersector; the two rows are one consolidation question —
which crate homes the planar arc-aware primitives, and whether
`profile` (2-D, `Seg`) and `topo` (3-D, loop edges) can share them
through `ray_parity`'s `RaySpace` abstraction the way the polygon walk
already does.

**Not measured:** whether the two copies disagree anywhere. Both are
exact constructions with graze-on-ambiguity; a drift would show as a
different graze set, not a different answer.

