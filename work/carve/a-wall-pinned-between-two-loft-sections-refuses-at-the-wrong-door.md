---
id: a-wall-pinned-between-two-loft-sections-refuses-at-the-wrong-door
kind: issue
title: a loft section hinged on one of its own edges collapses that wall, and refuses at the attach gate (or as NoParameterStep on strip 0) rather than naming the pinned wall
status: open
opened: 2026-10-06
priority: P2
cost: M
refs: [loft-v-parameterization-is-the-first-strips-so-a-rolled-section-changes-the-body]
---


Found by `carve/one-door-for-coincident-sections`'s sweep (2026-10-06).

A loft section rotated about one of its own edges, so that edge is
the same world segment in it and its neighbour, passes the stacking
fold (`crates/sweep/src/loft.rs` `stacking_fold`: the centroid moves
along the base normal) and leaves the wall over that edge with two
identical section curves. What happens next depends on which strip
the edge is, not on the fact:

- **The edge is segment 0 of loop 0** (the strip
  `first_strip_parameters` measures v on): the parameters do not
  ascend and the skin refuses `SkinError::NoParameterStep` naming the
  section. True, but it says nothing about the wall, and the same
  section authored from another start vertex takes the next arm.
- **Any other edge**: the parameters ascend, the skin builds, and the
  assembly refuses at the attach gate. Measured on three 2×2 squares
  at z = 0, hinged about the edge y = +1 by −0.3 rad, z = 2, v-degree
  2: `an Euler operation of the assembly refused: geometry attachment
  gate: the stored interval's span (not a sampled check) escalated:
  margin -8.468018830476105e-1 lies past the ambiguity band (1e-9,
  1e-8). Recourse: move the geometry so this edge is not vanishingly
  short` — a refusal about an edge's span, past the band, for a wall
  that collapsed to a segment.

The fixture is `one_door_for_coincident_sections.rs`'s hinged row,
with the hinge moved off strip 0. Whether a pinned wall should refuse
at all (a loft that touches itself along an edge) or build, and with
what refusal, is a design question; the v-parameterization item
(`refs`) changes which arm strip 0 takes.
