---
id: a-two-carrier-face-pair-off-its-tangent-line-reads-contradicted-not-stale
kind: issue
title: The carry refuses a divided cell's face–face Found row Unread where the two-carrier confirm reads a pair Contradicted, not stale
status: closed
branch: contacthold/ff-divided-reads-stale
pr: 5390
opened: 2026-10-13
priority: P2
refs: [a-carried-edge-face-record-the-census-cannot-read-refuses-every-later-op, point-touch-reading]
closed: 2026-10-19
---


Found by `contacthold/point-touch-reading` (the point-touch unit), re-running
PR 4921's review probe `review_4921_both_faces_divided` (branch
`contacthold/face-face-touch-row-review`) once the cap circles'
edge–face records read.

## What

A rod resting on a plate's side face, with a block crossing the rod's
wall (or a slot crossing it) and another crossing the plate's face. On
main these ops refused `CarryRefused { Found, Unread }` on a cap
circle's edge–face record. With the point-touch reading they refuse
`CarryRefused { Found, Unread }` on a **face–face** pair instead:

| op | pair | the confirm's finding |
|---|---|---|
| divided out, B∖A | `[Face(3), Face(15)]` | `ContactContradicted`, `contact_tangent_opposed` |
| divided in, A∖B | `[Face(5), Face(11)]` | `ContactContradicted`, `contact_tangent_overlap` |
| divided in, A∩B | `[Face(8), Face(16)]` | `ContactContradicted`, `contact_tangent_opposed` |
| divided in, B∩A | `[Face(3), Face(6)]` | `ContactContradicted`, `contact_tangent_opposed` |

(Keys from that probe's run; the finding read by printing the error
`records::landings` drops at its `map_err(|_| CarryRefusal::Unread)`.)

`census::confirm_face_pair` (`crates/topo/src/census.rs`) hands a pair
on two carriers to `boolean::tangent_pair_at_rest`, whose
`contact_verify::tangent_locus_relation` refuses `Contradicted` on the
material sides (`contact_tangent_opposed`) or the overlap
(`contact_tangent_overlap`) before the line is placed in either face.
The carry (`records::landings`) treats every finding but staleness as
unread, so a pair drawn from a divided cell's images whose sides agree
(a piece the op turned, or a cavity face) refuses the op where it is
not a meeting at all.

## Owed

Say which it is: a pair drawn from a divided cell's images whose
tangent reading contradicts is not a meeting (the carry passes over
it, as it passes a stale pair), or the confirm reads it stale where
the line misses a face before reading its sides. A fixture pinning
the four ops above.

## Re-measured on main (2026-10-19) and resolved

PRs 4996, 5006 and 4982 left all four ops refusing as above. The
confirm read the sides first. `census::confirm_face_pair` now places
the line in both faces (`boolean::rest_line_in_both_faces`, over
`boolean::line_through_both_faces`, the one shared reading) before the
`Tangent` table reads the sides (`boolean::tangent_table_at_rest`, split
out of `boolean::tangent_pair_at_rest`). The table takes only the value
the placement returns where the line runs through both faces
(`boolean::RestLineInBothFaces`), so the order holds by type. A pair
the line misses is stale,
whatever its sides. A pair the line runs through still contradicts on
wrong sides. Three of the four ops now answer right:

| op | now |
|---|---|
| divided out, B∖A | 2 solids, volume right, no face–face record (the plate is not in the result) |
| divided in, A∩B / B∩A | 2 solids, volume right, no face–face record |
| divided in, A∖B | still refuses `Unread`, a different reading: the line runs through both pieces and the table contradicts the crescent |

The fourth is the bore tangent to the plate's face from inside. The
op mints a record for that solid pinch, and its own 3′ already
contradicts the record on the battery's `p×c in` without any division.
That is a design fork, filed as
`a-bore-tangent-to-a-plates-face-mints-a-face-face-record-the-census-contradicts`.
Pinned:
`face_face_touch::a_record_whose_faces_the_op_divides_lands_on_the_pair_holding_its_line`
(every op of both fixtures but that one, both operand orders).
