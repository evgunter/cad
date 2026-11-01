---
id: an-edge-running-past-a-face-along-coincident-edges-is-not-divided
kind: issue
title: A Boolean edge running past the face it lies in, along another edge on its line, refuses EdgeInFace Unplaced instead of dividing
status: open
opened: 2026-10-21
priority: P2
cost: M
---


Found by the lane that builds the parting of an edge running past its
face (`cleave/boolean-parts-an-edge-running-past-its-face`, the row
`a-boolean-edge-running-past-the-face-it-lies-in-is-not-parted`).

## What

`topo::boolean::slit::divide` (`crates/topo/src/boolean/slit.rs:1034`)
divides an edge lying in a face along part of its length where it
crosses the face's boundary: at an edge of the loops an edge-edge record
names, or at a vertex of them a vertex-on-edge record names. It places
each vertex-on-edge record of a divided edge on the piece holding the
vertex. Any other record naming the edge or a crossed edge refuses
`BooleanError::EdgeInFace { gap: EdgeInFaceGap::Unplaced }`
(`slit.rs:1196`), since which piece holds it is not read; an
edge-edge record with an edge of the face's own boundary on the edge's
line refuses `Overlap` before any division.

`crates/topo/tests/three_solids_on_one_line.rs`, `UNPARTED`: the line
body (the plate and the prisms over 0°–50° and 120°–170°, touching
along `x = 1.5, y = 1`) less the 0° prism against the bisector, the
line body first, every member order. The prism's edge on the line,
`z ∈ [0.5, 1.81]`, lies in the bisector prism's face above the plate's
top, crossing the face's boundary at its corner `(1.5, 1, 1)` (a
vertex-on-edge record, divided), and overlaps the line body's own
coincident edges along `z ∈ [1, 1.7]`: edge-edge records between edges
on one line, which `divide` refuses. Main refused these cells
`LineWedges`.

## Fix

Place an overlap record of a divided edge on the pieces it overlaps,
read against the crossings along the line as a vertex-on-edge record
is; then the parting reads a line holding more than the two coincident
edges it makes, which the census's line reading must take
(`census::line_wedges`).
