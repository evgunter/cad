---
id: three-member-coincidence-rows-may-not-be-order-free-once-a-fourth-member-covers-them
kind: issue
title: DM4's three-member coincidence row may not be one row in every member order once a fourth member covers the point (intersect pinches, union glues)
status: open
opened: 2026-10-18
priority: P2
cost: M
design: true
refs: [a-union-fold-publishes-no-pinch-row]
---


## Finding

Raised by a FORK-FOLDPINCH designer (2026-10-18), reasoned, unmeasured.
For a union every pinch is some pair's pinch (near the point the result is
the union of the members' local cones, so two members in different pieces
pinch on their own), so the judgement holds every union pinch row. For
intersect that argument fails: with non-convex local cones three members
can pinch together where no two do, and DM4's rule for such a row ("a
three-member row is the union's row too, spelled by its member cells, so
it is one row in every order") may not hold once a fourth member covers
the point: in one order the third member's step decides it, in another the
fourth has already removed the region and no step does. The same may hold
for a three-member glue row. Measure first: three intersect members whose
local cones pairwise meet in one piece each but all three together in two,
plus a fourth covering the point, over every member order. If real, the
fix likely changes DM4's sentence, which is Ev's.

The other designer added a clarification DM4 is silent on: a fold row
whose cell the fold itself minted (a member corner landing on a seam
junction of two earlier members) — spell it by its canonical head in the
node's space. Unsure whether reachable.

Signed (WIRE orchestrator).
