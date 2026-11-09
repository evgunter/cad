---
id: a-declared-wall-is-read-only-by-a-vertex-crossing-it
kind: issue
title: A declared wall is read as one only by a vertex crossing it: a vertex touching it, a vertex pairing on it, and an edge lying in it still read its two faces apart
status: open
opened: 2026-11-09
priority: P2
cost: H
refs: [a-vertex-piercing-two-declared-coincident-faces-reads-as-one]
---


Filed by SEAL's lane on `a-vertex-piercing-two-declared-coincident-faces-reads-as-one`
from its sweep.

## What

The fork `seat-vertex-one-interface` rules that a face–face record's
two faces are one two-sided wall to every reader of that operand's
material at a point: the vertex cone, the line wedge, face containment
and the coplanar sector. The unit built the vertex cone at a crossing
only (`vtxfac::Wall`): a vertex whose reads of the two faces each
cross, and mirror each other run for run. Measured on the bolt-on-plate
seat with its record carried
(`crates/topo/tests/a_vertex_through_a_declared_seat.rs`,
`a_vertex_touching_a_seat_or_pairing_on_it_reads_twice`), the rest
still refuse `VertexReadTwice` in every op:

- **A vertex touching the wall** (all its edges on one side, or in the
  wall). The ridge whose apex edge lies in the seat touches both faces
  at its ends. Admitted as a wall vertex during the unit's build, the
  touch read through and `seat − ridge` built a body (volume 0.957, the
  right one) whose own records tier 3′ refuses: nine
  `UnrecordedContact`s, the bolt's foot corners on the plate's top,
  which the carried seat record no longer covers once an edge of the
  result lies in the seat. So the touch is refused at the wall
  (`vtxfac::classify_vertex_on_face`, "where the vertex only touches the
  wall"), and the line wedge reader (an edge lying in both faces,
  `recl`, `slit`) and the carry of the seat onto a face slit along the
  wall are what this row owes.
- **A vertex pairing on the wall.** A block beside the bolt's foot
  corner `(0.2, 0.2, 0.2)`, an edge through it: the edge's vertex on the
  seat pierces the plate's top and pairs with the bolt's corner, whose
  foot lies on that face. `vtxfac::partner_side` reads the partner's
  link against the pierced face alone, not the wall it is part of.
  `vtxfac::pierced_and_paired` likewise refuses a vertex that pierces a
  wall and pairs.
- **A record of curved faces.** Only a record of two planar faces is
  read as a wall (`vtxfac::Wall`), and only such a record is held
  apart at the result gate (`ops::Held::apart`): planar faces a record
  holds apart lie on one plane, over an area, while a curved record
  may be a tangency along a line (a rod resting on a plate, which
  `sweep`'s `face_face_touch` bridges and tier 3′ reads inside one
  shell). A coaxial area seat (a shaft in a bore, sphere on sphere)
  is a wall the readers do not yet read: a vertex through it still
  reads twice, and a union bridging it is not refused at the gate.
  Telling an area record from a tangency without re-deciding the
  carriers is part of this row.
- **A vertex whose reads part from the mirror**: an edge of it lying in
  the wall (On both faces), a sector flush with the wall, a run with no
  edge. The coplanar sector reader (Eq. 15.3's lump) reads each face
  apart.

## Done when

Each case above builds on the whole body at tier 3 and 3′ with the
seat's record carried, or refuses typed for a stated reason, and the
row's pins move with it.
