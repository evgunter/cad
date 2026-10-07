---
id: a-record-on-a-curved-edge-resting-on-a-face-refuses
kind: issue
title: A carried record that lands on a curved edge and a face refuses JoinDesync: no record kind stores a circle resting on a plane at a point
status: open
opened: 2026-10-07
---

## The finding

`topo::boolean::ops::record` (`crates/topo/src/boolean/ops.rs`, the
`(Cell::Edge, Cell::Face)` arm) consumes an edge resting on a face's
interior with no stored kind because "a line meeting a plane either
lies in it or pierces it". That argument is a line's and a plane's, so
the curved join made the arm refuse `JoinDesync` for a curved edge too:
a circle can rest on a plane at one point, which no record kind stores.
It arises when a vertex-on-face record's vertex is joined away into a
curved edge, or a conventional vertex's record is written on its edge.
No `ci` row reaches it.

## What it needs

An edge-on-face point record kind (or a census lane that certifies an
edge-face touch at a point from the body alone), so the pair carries
instead of refusing.
