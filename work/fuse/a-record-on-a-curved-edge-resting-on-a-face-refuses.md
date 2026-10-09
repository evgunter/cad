---
id: a-record-on-a-curved-edge-resting-on-a-face-refuses
kind: issue
title: A carried record that lands on a curved edge and a face refuses CurvedRestUnrecorded: no record kind stores a circle resting on a plane at a point
status: open
opened: 2026-10-07
priority: P1
cost: H
design: true
---

## The finding

`topo::boolean::ops::record` (`crates/topo/src/boolean/ops.rs`, the
`(Cell::Edge, Cell::Face)` arm) consumes an edge resting on a face's
interior with no stored kind because "a line meeting a plane either
lies in it or pierces it". That argument is a line's and a plane's, so
the curved join made the arm refuse typed,
`BooleanError::CurvedRestUnrecorded` (pncad-py tag
`curved_rest_unrecorded`), where the edge or the face is curved
(`ops::tests::an_edge_on_a_face_is_structure_only_on_a_line_and_a_plane`):
a circle can rest on a plane at one point, which no record kind stores.
It arises when a vertex-on-face record's vertex is joined away into a
curved edge, or a conventional vertex's record is written on its edge.
The census still sweeps a conventional vertex's point, so such a touch
left unrecorded reads as an undeclared vertex-on-face contact.
No `ci` row reaches it.

## What it needs

An edge-on-face point record kind (or a census lane that certifies an
edge-face touch at a point from the body alone), so the pair carries
instead of refusing.
