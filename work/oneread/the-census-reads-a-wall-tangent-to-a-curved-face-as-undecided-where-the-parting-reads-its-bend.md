---
id: the-census-reads-a-wall-tangent-to-a-curved-face-as-undecided-where-the-parting-reads-its-bend
kind: issue
title: census::edge_in_curved_face reads a wall tangent to a curved face as undecided, where the parting reads its second-order bend: one question, two answers
status: open
opened: 2026-10-31
priority: P2
cost: M
refs: [5793]
---

Found by the re-review of PR 5793 (CLEAVE). Unverified: no witness
has been built.

## What

`crates/topo/src/census.rs` `edge_in_curved_face` (`census.rs:4408`)
reads the sides of a line edge lying in a curved face of its own
solid. Where a wall of the edge is tangent to the face at first order
(`pm_census_ef_curved_wall_side` reads `Zero`), it reports the edge
undecided (`FaceThroughEdgeWedgeUndecided`, `TouchUnread::InBand`),
with no second-order reading.

The Boolean's parting asks the same question of the same edge
(`crates/topo/src/boolean/slit.rs` `walls_inside`, the tangent arm at
`slit.rs:1338`). There a tangent wall's side is read at second order,
`bool_rider_wall_bend_across`: a wall bending away from the face's
material leaves the face alone. So a same-solid body whose wall bends
away along a ruling of its own curved face would build at the parting
and then refuse at the census's tier-3′ pass. One question, two
answers (D3, one reading per question).

## Fix

Give the census the parting's second-order reading, ideally one shared
function of the two sheets' bends across the line, and build the
witness: a same-solid body with a wall tangent to its own cylinder face
along a ruling and bending away from it.

## Slate triage

2026-11-01: moved from `work/cleave/` by CLEAVE's split on its semantic seams (Ev, in chat: CLEAVE measured 114 budget points against 30). ONEREAD holds the boolean questions read by two homes; id and body unchanged. (CLEAVE orchestrator)
