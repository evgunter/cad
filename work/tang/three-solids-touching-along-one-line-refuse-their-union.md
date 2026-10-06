---
id: three-solids-touching-along-one-line-refuse-their-union
kind: issue
title: Three solids touching along one line refuse their union: the third solid's edge meets a doubled contact edge
status: open
opened: 2026-10-06
priority: P1
cost: H
---

## What

Found while closing
`three-wedges-meeting-at-a-point-on-a-face-refuse-in-every-member-order`.

The plate `[0,3] × [0,2] × [0,1]` with three vertical triangular prisms
over the sectors 0°–50°, 120°–170° and 240°–290° of radius 0.4 about
(1.5, 1). Their z-ranges are staggered: (0.5, 2.0), (0.47, 1.7) and
(0.44, 1.81). The three prisms touch pairwise along the vertical line
through (1.5, 1), so after any two of them are joined the line is held
as two coincident edges. The plate's top meets the line at one vertex,
which all three footprints share.

Measured on 443f33b7 as left folds of `topo::union` over the
24 member orders:

- **The third prism's step**, in every order but the six that put the
  plate last, refuses `ClassificationInvariant { "two distinct
  same-solid bounds share one ray (degenerate operand)" }`. That is
  `boolean/recl.rs` `recl_edges`: the accumulated body has two edges
  on one ray from the vertex, and the third prism's edge runs along
  that same ray.
- **The plate after all three prisms** refuses `JoinDesync { "a pinch
  face runs through a pierce vertex twice" }`.
- **The three prisms alone** build in topo, as [15, 33, 24]. Through
  editor-core's union the same orders refuse
  `Naming(SeamVertexPartners)`
  (`names/emit_topo.rs`, `one_partner`): a vertex with two distinct
  contact-record partners.

Two prisms along the line build in every order (`union_pinch_member_order`'s
corner-holes rows). So does every count of holes meeting at one vertex
when the prisms lean apart and share no line
(`crates/topo/tests/holes_meeting_at_a_vertex.rs`). What is missing is a
contact line held by three or more solids: an operand whose vertex
has two or more coincident edges on one ray, met by another solid
along it.

The item's own 60° sectors put a lateral face of one prism in the
plane of another's (0° with 180°, and likewise for the other two
pairs), touching along the line, which is an undeclared flush
continuation. Every order refuses `UndeclaredCoincidence` before the
join, and D10's `unproven-coincidence` finding replaces that refusal
when D10 is built (`work/recipe/d10-one-way-to-say-intent-is-unbuilt.md`).
This row is about the 50° fixture, which has no coincidence and only
contact.

## The coincidence door

This row touches the coincidence door. Whether the third solid's edge
glues to the doubled contact edge is a margined verdict under D10, so
the row is not wholly held ground. The fix decides a coincident edge
on one ray at the vertex-vertex lane rather than refusing it as a
degenerate operand. It must not lean on a declaration, and it should
be checked against D10's `unproven-coincidence` door when that is
built.
