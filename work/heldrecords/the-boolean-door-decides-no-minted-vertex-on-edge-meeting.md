---
id: the-boolean-door-decides-no-minted-vertex-on-edge-meeting
kind: issue
title: The boolean's carry door decides no minted vertex-on-edge meeting, so a carried record reaching one refuses JoinDesync
status: open
opened: 2026-11-25
priority: P1
cost: M
---


Found by the `heldrecords/removed-members-red` lane. Ruled (C) by the
HELDRECORDS orchestrator, with this as (A).

## What

The boolean's carry (`boolean::ops` `carried`) decides a minted meeting
only when it is two vertices (`vertices_meet`, `decide_one`). The README
carry clause says the boolean's tool "decides every such meeting at its
door". `a_pierce_held_in_a_curved_face::a_plane_face_holding_the_pierce_over_a_cylinder_builds_in_every_op_and_order`
reaches one it does not decide, and refuses `JoinDesync`:

- An edge-in-face record whose edge the cut splits at a vertex `v`.
  `v` lies strictly inside a section edge `s` minted in the face, not
  at either of its ends.
- The record also lands on the edge and the kept face piece.
- The row passed before PR 7745 named the section and has been red on
  main since.

Setting the meeting aside beside the record's other landing ((C)) was
built and taken back out (PR 7799): it changes the ratified clause.
The door decides the meeting instead (below).

## Way through

Decided (HELDRECORDS orchestrator, after two designers and a probe
that found no margin amplification), for a lane after PR 7799:

- **Decide the meeting at the door with the census's interior
  reading.** The boolean's carry decides a minted vertex-on-edge
  meeting the way the census reads a vertex inside an edge, at the
  band.
- **Write its decision row on the record's own input cells `(E, F)`**
  (the edge and the face of the edge-in-face record), not on the
  minted cells, so the row names cells an input holds without a
  `Mints` row for the section edge
  (`a-ring-vertex-minted-in-the-pierced-face-has-no-mints-row` is not
  needed for it).
- **Move the vertex–vertex door's row to the record's cells too.** A
  minted vertex–vertex meeting then names `(E, F)` as well, so section
  vertices can be named on both the carried and the reduction paths
  (`a-carried-records-section-vertices-are-not-named-in-the-booleans-division`).
- **Add a stress row** where `X` grazes `F` at the section: the
  meeting decided at the band's edge, refused in band.

With it,
`a_pierce_held_in_a_curved_face::a_plane_face_holding_the_pierce_over_a_cylinder_builds_in_every_op_and_order`
builds, its meeting landing and citing the decision.
