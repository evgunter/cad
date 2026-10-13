---
id: a-section-past-a-corner-lying-on-its-own-bodys-face-refuses-by-slot
kind: issue
title: A section along a contact line past a corner lying on its own body's face refuses VertexReadTwice in one operand slot and UnpairedLooseEnds in the other
status: open
opened: 2026-10-13
priority: P2
cost: M
refs: [a-contact-line-whose-ray-holds-a-subdivision-bisector-refuses-unbuilt]
---


Found by the TANG lane that built the contact-line bisector arm
(`a-contact-line-whose-ray-holds-a-subdivision-bisector-refuses-unbuilt`).

## What

`crates/topo/tests/three_solids_on_one_line.rs`,
`a_prism_against_a_contact_line_on_a_bisector_builds_or_refuses_typed_in_every_op`.
The line body is the plate, the third prism (side face through the
line) and the 120° prism, whose edge on the line lies in the third's
face from z = 0.47 to its corner at z = 1.7. A prism over 40°–110°
crosses the third's face along the line, so its section runs up the
line from the plate's top (z = 1) to the third's top (z = 1.81), past
the 120° prism's corner, which lies on the third's face with no
vertex of that face there.

Every op, in every member order of the line body:

- line body first: `VertexReadTwice` (the prism's vertex at z = 1.7
  read as a pierce of the third's face and as a pair with the corner),
  in all 18 cases;
- prism first: `VertexReadTwice` where the third is not the line
  body's first member (12 cases), and
  `Join(UnpairedLooseEnds { count: 2 })` where it is (6 cases).

On main these 6 refused at `recl_edges` "a contact line's ray holds a
subdivision bisector (unbuilt)". With the arm built, the plate's top
reads the crossing germ against the third's half-turn
(`recl::read_edge_edge`). The vertex pair at z = 1.7 holds only the 120°
prism's sectors, not the third's face, so it reads no germ there, and
the section's two null edges are left unpaired. So the double read the
other slot refuses typed goes undetected in this one. Without the 120°
prism every op builds sound, and so does the 0° prism, which lies
inside the third's half-turn.

The same blindness can occur at a single vertex. `recl_edges` gathers a
solid's wedges about the ray from its mentions: its bounds read On.
Where that solid holds a real edge on the ray and also a face sector
the ray runs inside, with no bisector there, the face is no mention.
The event then reads as a lone edge-edge pair that cannot see the face.
At the plate's top in this row the third's face is parted along the
ray by its bisector (the plate's top meets it at right angles), so the
arm sees it. A plate whose top slants across the line would leave the
ray inside one sector of that face. That case is not yet measured.

## The shape to give

At minimum, the double read should refuse alike in both slots and every
member order (`vtxfac::refuse_sector_rereads`): find why it misses
where the third leads the fold. Building the shape needs the vertex
pair at the corner to read the face its corner lies on, as the pair at
the plate's top does through the bisector. When it builds, the row's
`crossing` table goes.
