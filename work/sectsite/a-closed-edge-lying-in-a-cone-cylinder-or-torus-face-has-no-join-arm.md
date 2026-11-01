---
id: a-closed-edge-lying-in-a-cone-cylinder-or-torus-face-has-no-join-arm
kind: issue
title: A closed edge lying in a cone, cylinder or torus face refuses at the crossing layer, and the one-site join has no arm to side its conic there
status: open
opened: 2026-10-14
priority: P1
cost: H
refs: [5159]
---



Found by PR 5159's dual review, filed on its fix pass (branch
`paths/in-face-section-self-loop`): the residue of
`closed-in-face-section-loop-has-one-site`.

## What

A one-segment circle (one edge, one vertex) lying in a face of the
other operand joins as a one-site section loop where that face is a
plane or a sphere (`boolean/join.rs` `closes_at_its_site`). A pierce of
the face is sided by `chord_join.rs` `RimSide::of`, which both the join
and `join_lone_ring` read: a plane cuts a sphere in the conic alone, so
the conic's plane sides its two caps. A cone, a cylinder or a torus has
no arm there, because the conic's plane can cut the face in more than
the conic.

That refusal is not what a caller sees today. The pose refuses earlier,
at the crossing layer (`boolean/reduce.rs`, the curved sweep arm's
`CurvedPierceUnsupported` frontier):

- a triangle revolved about the axis straddling a cylinder's wall or a
  cone's, its vertex sweeping a circle in that face, refuses
  `CurvedPierceUnsupported` in all six ops (pinned by
  `crates/sweep/tests/a_closed_conic_in_the_partners_face.rs`
  `a_closed_edge_in_a_cylinder_or_cone_face_refuses_typed`; the
  reviews measured it at the default ε, 1e-6 and 1e-12);
- a straddle on a torus's tube circle refuses `CurvedPierceUnsupported`
  too, and a disc lying on a torus's top parallel refuses
  `CurvedBooleanUnsupported` (the reviews' probes).

## What a fix has to supply

Two layers, in order: the crossing layer has to take a closed edge
lying in the curved face (an on-carrier conic), and then the join needs
a `RimSide` arm for the face, deciding which of the conic's two regions
holds the face's outer loop where the conic's plane can meet the face
again. Once the crossing layer takes the pose, the join refuses it
`SingleSiteSectionLoop` until the arm is built: `closes_at_its_site`
leaves the record for that refusal, reading the same `RimSide::of` that
`join_lone_ring` sides by, so the two cannot disagree. No pose reaches
that refusal today.

## 2026-10-29: the cylinder's and the cone's straddles build (SPLITDOOR)

With the carrier ladder's cone arm (`splitdoor/curved-copy`), the straddle's cone faces are decided distinct from the wall, so the crossing layer reads its circle as an edge lying on the wall's carrier (`reduce::curved_face_arm`'s `LiesOn` lane, `parents_distinct_from`). Every op, both orders, then builds at its closed form, tier 3 and 3′, one shell, meshed: against the cylinder (`2π`, straddle `π/2`, overlap `5π/24`) and against the cone frustum (`13π/6`, overlap `3π/32`). No `SingleSiteSectionLoop` appeared. Pinned as builds by `crates/sweep/tests/a_closed_conic_in_the_partners_face.rs` `a_closed_edge_in_a_cylinder_or_cone_face_holds_every_op`. The torus's straddle and the disc on a torus's top parallel were not re-measured.
