---
id: a-meeting-of-three-members-at-one-point-has-no-order-free-sense
kind: issue
title: Where two members' edges meet on a third member's face, or three members' edges meet, the senses an EdgeCrossing would carry depend on member order
status: open
opened: 2026-10-12
priority: P2
cost: M
---



## What

This is the residue of the touch ruling (Ev, PR 4284): "three or more
members' edges at one point". Ev leans to A's reading, unsure: read
each edge's sense against all the other members together. The build
(branch `emit/touch-crossing`) does not do that. Why:

- **No spelling holds three edges.** `EdgeCrossing { a, a_sense, b,
  b_sense }` carries two. At a point where k ≥ 3 members' edges meet,
  a sense per edge needs a segment with k sides, which widens a ratified
  spelling. Today's spelling there is the junction run of seam lines.
  The union now reads that run from the finished body
  (`emit_union::Flush::junction`), so it does not depend on the fold.
- **Two members' edges on a third member's face.** Here the two-edge
  spelling applies (`Flush::edge_crossing`), but the senses it carries
  are the ones the fold step that minted the vertex read. That step
  read them against the fold so far, so they depend on which members
  that held.

**Measured** (`crates/editor-core/tests/union_pinch_member_order.rs`,
`tilted_holes`, the plate and leaning wedges whose legs meet at its top,
every member order):

| fixture | names on main | names on the branch |
|---|---|---|
| two wedges and the plate | 3 | 3 |
| three wedges and the plate | 7 | 1 (6-line junction) |
| four wedges and the plate | 15 | 1 (8-line junction) |
| two wedges | 1 | 1 (`EdgeCrossing`, both `Outside`) |
| three wedges | 3 | 1 (6-line junction) |
| four wedges | 10 | 1 (4-line junction) |

Two wedges and the plate:
- where a wedge meets the plate before the other wedge, the vertex is
  spelled `Crossing { leg, plate's top, sense }` with that wedge's leg;
- the plate folded last spells it `EdgeCrossing { leg, Outside, leg,
  Outside }`, read against the other wedge alone.

Against everything else at the point, each leg enters or leaves the
plate there.

## The question

Should the k ≥ 3 case, and the two-edge case with a third member's face
through the point, be spelled by every edge with its sense against the
union of the others? If so, in what segment? Or should both stay with
the finished body's junction run, which the two-edge case would join?
A design fork, for the two designers before Ev.

## What this row owns (review of PR 4816)

**1. The README's flush paragraph is false for a reachable two-edge case.**
The paragraph says "one where two member edges meet is their
`EdgeCrossing` … so every member order spells it alike". Two wedges
and the plate do not: two member edges meet there, on a third member's
face, and the union publishes three names.

The mechanism is `emit_union::Flush::edge_crossing`'s no-sense
fallback:
- Where a wedge meets the plate first, that step mints the vertex as
  the leg's `Crossing` of the plate's top.
- When the second wedge folds in, its leg reaches that vertex as a
  vertex of the fold so far. The corner rule names such a vertex as the
  fold's own (N2), so no step reads a sense along the second leg.
- With no sense to read, the fallback publishes the fold's `Crossing`.

This is not a regression (main had three names here too), and it is a
two-edge case, not the three-or-more residue. Pinned by
`union_pinch_member_order::two_legs_on_the_plate_are_spelled_by_member_order`.

**2. The README has no sentence for the finished-body junction reading.**
The code (`Flush::junction`) has no counterpart in the README. The
missing sentence would state:
- **Which lines count:** each finished edge at the vertex that lies
  along no member edge and runs between faces of different members,
  spelled `Seam` of its faces' parent names in name order, the run
  sorted.
- **Where it applies:** where three or more member-edge lines meet, so
  where member edges also meet, unlike the pair emitter's junction,
  which names a point no operand edge reaches.

The README is left for the designer pair and Ev.

**3. The union's silent fallbacks.** Each publishes the fold's spelling
with no signal. They are named in the doc comments of
`Flush::edge_crossing` and `Flush::junction`.

| fallback | reached by | row |
|---|---|---|
| `edge_crossing`: the two lines are one member's | the pinch's joined blocks against the plate, whose two coincident contact edges pierce the plate's top: `the_plate_against_the_joined_blocks_builds_in_every_op` and `a_slab_holding_the_contact_is_one_vertex_only_on_one_fragment`, 2 vertices each. The fold's four-line junction stands, alike in both orders. | `two_edges_of_one_member_at_the_pinch_keep_the_folds_junction` |
| `edge_crossing`: no step read a sense along one line | two wedges and the plate, the 4 orders where a wedge meets the plate first | `two_legs_on_the_plate_are_spelled_by_member_order` |
| `junction`: fewer than two seam lines | no fixture | none |
| `junction`: a face at a seam edge has no parent name | no fixture | none |

Refusing at any of these would break the pinch builds, so none refuses.

## Ruling (Ev, PR 4881, fork-log row 124)

The ruling is stated in names README N2 *Vertices* and the flush paragraph:
- **The name.** A vertex that is not an operand corner is named by the lines through it: the member edges cut there (flush lines counted once, as the least), plus, at a union, each seam whose faces bound none of those edges.
- **Spellings.** One line meeting one face is a `Crossing`. Two or more lines are an `EdgeCrossing` run.
- **Senses.** Each edge's sense is read against the members of the name's other entries, from the members' pairwise judgements. Seams carry no sense, and vertices with one run rank along a straight seam they share.
- **Retired.** `Flush::junction` and the four silent fallbacks.

The build is this row's. It also closes the order dependence in
`a-crossing-of-a-union-seam-is-spelled-by-member-order`.
