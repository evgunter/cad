---
id: a-prism-crossing-a-face-along-a-contact-line-refuses-vertex-read-twice
kind: issue
title: A prism crossing a face along an edge lying in it refuses VertexReadTwice, or UnpairedLooseEnds at the join in one slot; a wedge's face beside the edge refuses ClassificationInvariant
status: closed
opened: 2026-10-13
priority: P2
cost: M
refs: [a-contact-line-whose-ray-holds-a-subdivision-bisector-refuses-unbuilt]
branch: tang/crossing-face-vertex-read-twice
closed: 2026-10-14
---

Found by the TANG lane that built the contact-line bisector arm
(`a-contact-line-whose-ray-holds-a-subdivision-bisector-refuses-unbuilt`).
Two of the witnesses below came from that PR's review (PR 5018).

## What

The witnesses live in `crates/topo/tests/three_solids_on_one_line.rs`.
The line body is the plate, the third prism and the 120° prism; the
third prism's side face holds the vertical line through (1.5, 1). The
120° prism's edge lies in that face from z = 0.47 up to its corner at
z = 1.7. An upright prism over 40°–110° crosses the third's face along
the line.

Measured in every member order of the line body and every op, either
operand first, on main and on PR 5018 alike unless noted:

- The prism over z 0.5–2.0
  (`a_prism_against_a_contact_line_on_a_bisector_builds_or_refuses_typed_in_every_op`)
  refuses `VertexReadTwice` (a prism vertex on the line read as a pierce
  of the third's face and as a pair) in 30 of 36 cases. The other 6
  have the prism first and the third leading the line body's fold. On
  main they refused at `recl_edges`' bisector `ClassificationInvariant`.
  On PR 5018 they refuse at the join with
  `Join(UnpairedLooseEnds { count: 2 })`, whose message names no
  recourse and lists a kernel defect among its causes.
- The same prism over z 0.5–1.6, its top below the corner, refuses
  `VertexReadTwice` in all 36. A prism over 230°–300°, z 0.5–1.6,
  refuses it in all 36 too.

So the refusal does not need the section to pass the 120° prism's
corner. What the poses share is a section along the line that crosses
the third's face where the 120° prism's edge lies in it: the face holds
an edge of its own body along the section, with no vertex or edge of
the face there.

In the 6 join cases (prism first, order [2, 0, 1]), the vertex pair at
z = 1.7 holds only the 120° prism's sectors on the line body's side.
The prism's vertex there is read as a pair and never as a pierce of the
third's face (traced with a probe on `boolean_reduce`'s vertex-pair
loop). With the line body first, the same vertex is read both ways and
refuses `VertexReadTwice`. So the asymmetry is upstream of the
double-read check, in which vertices the vertex-on-face reading
reaches, and it is not a missing branch in that check.

### A face beside the edge, no bisector

Line body: the plate, the 120° prism, and a wedge whose face holds the
line, its apex at the plate's top with no bisector there
(`Wedge { half: 45, toward: 350 }` in the row file). Tools: upright
prisms over 0°–50°, 280°–330°, 40°–110° and 180°–230°, z 0.5–2.0.
Identical on main (the review measured both):

- Folding the line body refuses `VertexReadTwice` in the 2 orders with
  the 120° prism last.
- Union with the tool first refuses `ClassificationInvariant("two edges
  touching along a line are read at other than their two ends")` in the
  2 orders that fold the wedge last, for every tool.

It is the same blindness at one vertex. The plate's top holds the 120°
prism's edge and a face sector of the wedge that the ray runs inside,
not on a bound, so that face is no mention and `recl_edges` reads a
lone edge-edge pair there that cannot see it. The touch is then read at
one end of the line and not the other.

## The shape to give

- At minimum, every pose above refuses typed, alike in both slots and
  every member order: the double read is caught wherever it happens,
  and the touch disagreement becomes a typed refusal with a recourse
  instead of a `ClassificationInvariant`.
- Building it needs every vertex pair along such a section to read the
  face the section runs in, as the pair at the plate's top does through
  the bisector.
- When it builds, the row's refusal table goes.

## Closed

The asymmetry came from the sweep, not the double-read check.
`reduce::sweep_direction`'s line lane read a fully coplanar edge–face
pair at the edge's ends only. When a later face split that edge, the
vertex the split minted was never read against the face the edge lies
in: the pieces went on from the next face. The other slot mints that
vertex on the other operand, whose own sweep reads it from the first
face.

The worklist now carries the planar faces the line lane's coplanar arm
read each edge in. Every split reads its vertex against them in
`read_then_requeue`, as the ends were read, and the pieces inherit
them. A conic lying in a plane and an edge lying on a curved face are
not carried: `a-vertex-minted-on-an-edge-lying-on-a-curved-face-is-unread`. Every witness above refuses `VertexReadTwice` alike in every op,
slot and member order. The wedge's tool-first union, which refused
`ClassificationInvariant` on main, builds sound in every order, tier 3′
included.

`VertexReadTwice` now names its recourse: apply the operation to each
piece of the other solid that touches there, then combine the results.
`the_recourse_of_a_prism_crossing_a_face_beside_a_resting_corner_builds`
runs that route for every op at both tops, and it builds sound.
Building the shape itself is
`a-corner-read-as-a-pair-and-a-pierce-is-unbuilt`.
