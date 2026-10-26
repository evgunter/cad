---
id: the-edge-edge-and-point-record-confirms-read-no-material-sides
kind: issue
title: The edge–edge and point record confirms read only that the cells meet, never which sides their materials lie on
status: open
opened: 2026-10-19
priority: P2
cost: M
---


Found by both designers of the pinch-crescent fork (`[ev]` branch
`contacthold/ev-pinch-crescent`), off its question.

## What

D1 tier 3's principle is one check at every contact: the touching
cells' materials lie on opposite sides of each other (PR 4795). The
face–face confirm reads the sides (the `Tangent` table through
`tangent_locus_relation`, "material sides included",
`crates/topo/src/census.rs:7853`–`7869`). Two other record confirms
read only that the cells meet:

- `confirm_edge_edge` (`census.rs:8158`) decides that two edges'
  interiors cross or overlap (`interiors_meet`, `ee_meeting`, or
  `curved::curved_interiors_meet`) and nothing about the wedges'
  material sectors at the meeting.
- the point lane (`census/point.rs:313`, `point::face_face`, reached
  from `census.rs:7923`) confirms a sphere–plane or sphere–sphere touch
  as a point coincidence, with no reading of which side either
  material lies on.

So a recorded edge–edge or point meeting whose materials overlap
confirms as a contact where it is interference.

## Owed

Each record confirm reads the sides at its meeting: coincident edges'
material sectors disjoint, a point touch's two materials on opposite
sides of the common tangent plane, a shared region interference (C6).
A probe per lane: a recorded meeting of overlapping materials refuses.

## Measured (2026-10-19)

By both designers of the pinch fork's round 3 (`[ev]` PR 5395), on main
`f6ee5c7db8`: the plate `[-2, 2]² × [0, 1]` minus a rod r = 0.5 about
`(0, 1.5)`, z ∈ [0.25, 0.75], with the rod's seam placed on the
tangent ruling x = 0, y = 2. ∖ builds one solid whose seam edge lies in
the plate's y = 2 face, with an edge–face record (seam, plate face) and
two vertex–face records; tier 3 and 3′ pass. Beside the ruling both
faces' material sides hold the crescent, and the edge–face confirm
never asks. With the seam off the ruling the same body carries a
face–face record and 3′ refuses `ContactContradicted`
(`work/contacthold/a-bore-tangent-to-a-plates-face-mints-a-face-face-record-the-census-contradicts.md`).
The edge–face lane belongs in this row's list beside the edge–edge
and point lanes. Disc ∖ nested disc with its seams on the line (one
solid, v = 3π, two coincident seam edges, one edge–edge record) also
passes 3′ with no sides read.

## 2026-10-21: coincident line edges are read (RESTREAD, `restread/coincident-wedges`)

Pass 5's collinear lane now reads the two edges' material wedges
wherever it certifies a collinear overlap, a recorded pair included
(`census.rs` `ee_collinear_lane`, `ee_overlap_sides`, through
`census::line_wedges`), and refuses an overlap
`CoincidentWedgesOverlap`. So the edge–edge bullet above stands for
two curved edges (pass 5′,
`tier-3-prime-reads-no-side-between-coincident-curved-edges`) and for
a recorded crossing at a point, and no longer for two line edges
overlapping on one line; the disc ∖ nested disc's two coincident seam
rulings are read now. The record confirm itself
(`confirm_edge_edge`) still reads no side, which this row keeps.
