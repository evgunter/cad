---
id: a-meeting-of-three-members-at-one-point-has-no-order-free-sense
kind: issue
title: Where two members' edges meet on a third member's face, or three members' edges meet, the senses an EdgeCrossing would carry depend on member order
status: open
opened: 2026-10-12
priority: P2
cost: M
design: true
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
