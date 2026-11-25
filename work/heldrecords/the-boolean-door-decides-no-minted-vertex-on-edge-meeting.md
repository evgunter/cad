---
id: the-boolean-door-decides-no-minted-vertex-on-edge-meeting
kind: issue
title: The boolean's carry door decides no minted vertex-on-edge meeting, so a carried record reaching one refuses JoinDesync
status: open
opened: 2026-11-25
priority: P2
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

- An edge-in-face record whose edge the cut splits at a vertex. That
  vertex lies on a section edge minted in the face, where the section's
  own end vertex sits at the same point.
- The record also lands on the edge and the kept face piece.
- The row passed before PR 7745 named the section and has been red on
  main since.

Setting the meeting aside beside the record's other landing ((C)) was
built and taken back out (PR 7799). It changes the ratified clause, so
(C) and this door decision go to designers and Ev together.

## Way through

Decide a minted vertex-on-edge meeting at the door, as `vertices_meet`
decides two vertices: a point-to-curve gap at the band, and a pending
row in input cells. The section edge has no input cell yet. It needs
its own `Mints` row, as the ring vertex does in
`a-ring-vertex-minted-in-the-pierced-face-has-no-mints-row`. See also
`a-carried-records-section-vertices-are-not-named-in-the-booleans-division`.
