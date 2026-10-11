---
id: seam-cover-reads-a-boundary-point-as-the-face-interior
kind: issue
title: seam cover reads a touch piece on a face's boundary as its interior, and ships a decided SeamFaceRunsOn contradiction
status: open
opened: 2026-10-11
priority: P2
cost: E
---

Found by the review of PR 4613 (CLEAVE mints unit 4).

## What

The seam cover reads each touch piece of a locus at its midpoint, and
asks a face that does not ride the piece whether it contains it:

- `crates/topo/src/boolean/rim_wedge.rs:546` (`contains`) answers
  `Some(true)` for every placement but `Out`, so a midpoint the face's
  containment door places `OnEdge` or `OnVertex` (on its BOUNDARY)
  reads as contained.
- `rim_wedge.rs:344` (`departures`): a piece both faces contain where
  one has no riding edge is `Departure::NoEdge`, "the locus runs
  through its interior".
- `crates/topo/src/boolean/mod.rs:5806` and `:5903` turn `NoEdge` into
  the DECIDED contradiction `SeamFaceRunsOn` (`seam_locus_no_edge`).

So a midpoint the door reads on the face's boundary, not in its
interior, becomes a definite contradiction of the declared seam. A
boundary reading answers neither "interior" nor "outside" for the
question `NoEdge` asks.

## Repair shape

`contains` keeps the door's placement (`In`, `Out`, on the boundary),
and `departures` decides what a boundary midpoint means for the
piece: a riding edge the cut walk missed (a cut or ride defect), or a
piece the read does not place (`EdgeKindUnread`). Not a decided
`NoEdge`.

The arc-end case (`ContainError::ArcEndClearOfCorners`) already routes
to `EdgeKindUnread` (PR 4613).
