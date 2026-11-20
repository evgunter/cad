---
id: a-closed-edge-lying-in-a-cone-cylinder-or-torus-face-has-no-join-arm
kind: issue
title: A closed edge whose one vertex lies on the partner's wrap edge fell outside the one-site join's table
status: closed
opened: 2026-10-14
priority: P1
cost: H
refs: [5159]
closed: 2026-11-20
pr: 7177
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

What the join actually lacked (measured 2026-11-20): `boolean/join.rs`
`closes_at_its_site` had no arm for `(Wrap, Closed)` or `(Closed, Wrap)`.
That is a closed edge of one operand whose one vertex lies on the wrap
edge of a one-face closed wall of the other. The record fell to the
catch-all `_ => false` and was refused `SingleSiteSectionLoop`.
`RimSide::of` is never read for such a pose. A closed conic on a cone,
a cylinder or a torus winds round the surface, and an at-rest face of
one is a chart disc cut by its seams. So the conic meets a seam: at its
one site, on the wrap edge, which is this arm; or inside its span,
where the loop has two sites and already builds. A wall revolved from a
profile touching the axis is two half-turn faces, so a straddle there
crosses the π seam and has two sites (the cylinder and cone rows
below). A torus's or a tube's wall wraps the axis once, so its
straddle has one site, on the wrap edge.

## 2026-10-29: the cylinder's and the cone's straddles build (SPLITDOOR)

With the carrier ladder's cone arm (`splitdoor/curved-copy`), the straddle's cone faces are decided distinct from the wall, so the crossing layer reads its circle as an edge lying on the wall's carrier (`reduce::curved_face_arm`'s `LiesOn` lane, `parents_distinct_from`). Every op, both orders, then builds at its closed form, tier 3 and 3′, one shell, meshed: against the cylinder (`2π`, straddle `π/2`, overlap `5π/24`) and against the cone frustum (`13π/6`, overlap `3π/32`). No `SingleSiteSectionLoop` appeared. Pinned as builds by `crates/sweep/tests/a_closed_conic_in_the_partners_face.rs` `a_closed_edge_in_a_cylinder_or_cone_face_holds_every_op`. The torus's straddle and the disc on a torus's top parallel were not re-measured.

Moved from SECTSITE to SECT on 2026-11-02, by `git mv`, when SECTSITE merged back into SECT: the track budget now counts only the P0, P1 and unbanded rows a sitting dispatches (Ev, in chat, 2026-11-02), so SECT and the three tracks cut from it (SECTTAIL, SECTRUNG, SECTSITE) came in under one budget.

## Closed

PR 7177. `closes_at_its_site` admits a closed edge whose one vertex
lies on a one-face closed wall's wrap edge, `(Wrap, Closed)` either way
round, for every wall kind. The torus's straddles at five parallels and
a tube's straddle, each aligned with the wrap edge and turned off it,
build every op at their Pappus forms
(`a_closed_edge_on_a_torus_parallel_holds_every_op`,
`a_closed_edge_on_a_tubes_wrap_edge_holds_every_op`). The top-parallel
straddle is its own row
(`a-straddle-on-a-torus-top-parallel-refuses-at-the-plane-torus-chords-tangent-circle`).
