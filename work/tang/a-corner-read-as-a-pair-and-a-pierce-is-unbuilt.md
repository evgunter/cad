---
id: a-corner-read-as-a-pair-and-a-pierce-is-unbuilt
kind: issue
title: A corner read as a pair and as a pierce at one point refuses VertexReadTwice: classifying it against both is unbuilt
status: open
opened: 2026-10-14
priority: P2
cost: H
design: true
refs: [a-prism-crossing-a-face-along-a-contact-line-refuses-vertex-read-twice]
---


Filed by the TANG lane that gave `VertexReadTwice` its recourse
(`a-prism-crossing-a-face-along-a-contact-line-refuses-vertex-read-twice`).

## What

`BooleanError::VertexReadTwice` refuses wherever one operand's corner
is read twice at a point where the other solid touches itself: a
pierce of one of its faces and a pair with one of its corners, or two
pierces. The refusal is typed and names its recourse: apply the
operation to the pieces of the other solid that touch there, before
they are joined. The shape itself is not built.

Witnesses, in `crates/topo/tests/three_solids_on_one_line.rs`:

- `a_prism_against_a_contact_line_on_a_bisector_builds_or_refuses_typed_in_every_op`.
  The line body is the plate, the third prism and the 120° prism, whose
  edge lies in the third's face with its corner resting on that face.
  A prism over 40°–110°, its top at 2.0 or 1.6, crosses the third's
  face along the line. It refuses in every op, slot and member order
  (72 cases).
- `the_recourse_of_a_prism_crossing_a_face_beside_a_resting_corner_builds`:
  the same ops applied to the two pieces before they are joined build
  sound (10 routes).
- `a_contact_line_beside_a_face_holding_it_builds_in_every_op`: the
  plate, the 120° prism and a wedge whose face holds the line, folded
  with the 120° prism last, refuse at the fold. That prism's corner
  rests on the wedge's face.
- `crates/topo/tests/a_vertex_read_by_two_sector_passes.rs`: pyramids
  on their apexes at one point of a plate.

## The open question

How do a pair's sectors and a pierce combine at one corner? The vertex
pass reads the corner's link against the pierced face and hangs struts
(`vtxfac`). The vertex-pair pass reads it against the partner's
sectors (`recl`). Each writes the corner's orbit assuming it is the
only read there. Building the shape needs one reading of the corner
against both the face and the partner's sectors, and an insertion
that places the face's section and the pair's germs around one orbit
without either undoing the other.

## The shape to give

- A design choosing how the two reads combine, weighed per
  `memories/orchestration-model.md`.
- Then the arm. Each witness above builds sound in every op, slot and
  member order, or refuses typed where the design says it must.
