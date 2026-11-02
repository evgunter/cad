---
id: the-trimmed-lanes-closure-guard-cites-a-shoelace-the-rectangle-lane-no-longer-reads
kind: issue
title: props::quad::trimmed's closure-guard docs say the rectangle certificate's shoelace must equal ±the area, but rectangle_winding now reads segments
status: open
opened: 2026-10-30
priority: P3
cost: C
---


(QUAD 6013 fix lane, PR 6013.)

`crates/geom-brep/src/props/quad/trimmed.rs` says twice (the doc above the closure guard, around line 446, and the test doc around line 2758) that "the rectangle certificate refuses the analogous shape (its shoelace must equal ±the rectangle's area)". PR 6013 replaced that shoelace in `topo::props::quad_lane::nurbs_face` with `rectangle_winding`, which reads each iso segment's `(start, end)`: it refuses an open walk by requiring each segment to start within a few ulps of where the one before it ended, then reads the sides and their direction by comparisons. The claim still holds (an open walk refuses), but the mechanism the prose names is gone. Re-word both sites to cite `rectangle_winding`'s continuity check. This lane did not touch it because `props/quad/` is under a concurrent QUAD lane.
