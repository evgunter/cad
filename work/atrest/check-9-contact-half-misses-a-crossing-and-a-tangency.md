---
id: check-9-contact-half-misses-a-crossing-and-a-tangency
kind: issue
title: check 9's contact half does not see a ring CROSSING or TANGENT to its outer loop at a point that is a vertex of neither, and the nesting arm is premised on the crossing being absent
status: open
opened: 2026-09-24
priority: P1
cost: D
---


Found by ATREST-5, which had to state the premise check 9's nesting
arm places a whole ring on one vertex with.

**The premise.** `validate::ring_nesting` queries the ring's vertices
and takes the first definite verdict. That is sound exactly when the
ring and the outer loop do not CROSS: a ring disjoint from the outer
boundary is a connected curve in one component of the plane minus it,
so one point places all of it. The premise is checked nowhere the arm
runs:

- check 9's contact half (`validate::ring_outer_contact`) matches a
  shared vertex position, a vertex on an edge's interior, and an edge
  running along an edge — not a **transversal crossing** at a point
  that is a vertex of neither, and not a **one-point tangency**
  (circle-circle internal or external, line-circle). Its banner lists
  both as residue: *"Three-sample locus agreement cannot see a single
  shared point, and the closed forms that could (|c1-c2| vs |r1 +/-
  r2|) need an arc-containment test this predicate does not have."*
- the tier-3′ census (`census::sweep_edge_edge`) refuses an in-plane
  edge-edge crossing, but only between `Line` edges and only at
  `validate_pseudomanifold`'s door — `validate_geometric` never runs
  it, and no arc edge enters its snapshot (`census::edge_is_line`).

So on an arc-bearing ring or outer loop the premise is ASSUMED. The
shapes it lets through at rest: a ring crossing its outer loop whose
first decided vertex is inside
(`validate::tests::a_ring_crossing_its_outer_loop_passes_as_the_banner_says`
measures the line case); a ring arc bowing past a circular outer loop
while every ring vertex sits inside it (unmeasured — no public door
mints it, `Profile` validation refuses crossing loops); and a ring
tangent to its outer loop at a non-vertex point.

**A cheap exact third.** Where BOTH loops are `loop_shape`'s `Disc`
class the two loops are whole circles, so the arc-containment test the
banner says is missing is not needed: `|c_o - c_r|` against
`R - r` and `R + r`, each through one decide, classifies nested,
tangent, crossing and disjoint exactly. That is a contact-half arm
(the report is `RingMeetsOuter`, with a contact shape naming the two
loops rather than a vertex, since a crossing need not have a ring
vertex outside to name). Why it is not a nesting-arm test is argued
once, at `validate::ring_nesting`'s doc.
The general case (arcs of different circles, lines against arcs) needs
the arc-window test `boolean::contain::point_on_arc` already spells.

**Priority**: P1, not P0 — no normal verb is known to mint either
shape (the shell verb's own door refuses ahead of them, and profile
validation refuses crossing loops), so this is the at-rest statement
being weaker than it reads rather than a live wrong answer.
