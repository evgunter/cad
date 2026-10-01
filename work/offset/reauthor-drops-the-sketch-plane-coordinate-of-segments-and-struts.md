---
id: reauthor-drops-the-sketch-plane-coordinate-of-segments-and-struts
kind: issue
title: The axial door's re-author drops a PlacedSegment's or ExtrudedPoint's out-of-plane coordinate without a decide
status: open
opened: 2026-10-01
refs: [equator-seam-reauthor-refuses-the-hollowed-elbow]
priority: P3
cost: E
---

## What

Found by #3626's sweep. In `crates/topo/src/offset_axial.rs:reauthor`
(the `PlacedSegment` arm's `flat` closure, and the `ExtrudedPoint`
arm), the moved endpoint is pulled back through the placement and its
`z` (the distance off the sketch plane, a length) is dropped without
a decide. The `RevolvedPoint` arm decides the same coordinate
(`offset_axial_reauthor_plane` / `_end` / `_azimuth`).

The attach layer re-certifies the declaration against the carrier,
so a corner off the plane refuses there loudly rather than landing
wrong. The re-author itself cannot say which end left the plane,
though, and the dropped coordinate is a decision made by omission.
No door-built operand is known to reach it.

The planar door's `crates/topo/src/replace_face.rs:move_mapped_endpoint`
has the same shape (lines only) and lives on SHELL's ground.

## Fix shape

Decide `q.z` at each pulled-back end (a named predicate with an audit
row, also run at `T = Interval`), and refuse typed naming the end, as
the revolved arm's `_azimuth` does.

## Home

CURVED (`offset_axial.rs`); the planar sibling is SHELL's.

Moved from CURVED at its close (2026-10-01): `crates/topo/src/offset_axial.rs` is OFFSET's ground.
