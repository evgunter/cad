---
id: one-segment-tangent-rims-refuse-where-their-two-arc-forms-build
kind: issue
title: A boss over a hole whose rims are tangent refuses in its one-segment form where the two-arc form builds
status: open
opened: 2026-10-26
priority: P1
cost: H
---


Found measuring
`a-boss-over-a-hole-tangent-inside-its-rim-at-its-vertex-unites-as-two-shells`.
Pinned by `crates/sweep/tests/a_closed_conic_in_the_partners_face.rs`
`a_boss_over_a_hole_by_pose`, at the default ε, 1e-6 and 1e-12, alike.

The boss `r = 0.3`, `z ∈ [0, 1]`, over the plate `[−1, 1]² × [−0.5, 0]`
holed through or pocketed by a circle `r = 0.1`. Each rim is a
one-segment circle, or two arcs with vertices at the given azimuth and
opposite. Where the two-arc form builds `SOUND` in every op, the
one-segment form refuses:

| pose | one segment | two arcs |
|---|---|---|
| tangent outside (hole about `(0.4, 0)`), both vertices at the tangent point | ∪ `Join(SectionLoopMixed)`; ∩ and both differences build | every op `SOUND` |
| tangent outside, the boss's vertex at 1 | `CurvedPierceUnsupported` in every op | every op `SOUND` |

With no vertex at the tangent point at all (the boss's at 1, the hole's
at 2, tangent inside), every op refuses `CurvedPierceUnsupported` in both
forms: the crossing layer reads the tangency of a rim with the other's
wall.

Once PATHS unit 4 (`work/profile/circle-lowers-to-one-segment.md`)
lowers `circle` to one segment, every `circle()` puts its vertex at
azimuth 0, so these are the forms `circle()` authors.

## Why the one-segment form took another path

Two causes, one per row of the table above.

- **A closed edge's far end is its own site.** At a vertex pair, a germ
  whose sectors both hold a bound read On is placed by
  `boolean/sectors.rs` `germ_loci`, which reads what the reduction
  recorded at each edge's far end (`touch`): the two-arc rims' far
  vertices are the boss's antipode, lying in the plate's top (`Face`),
  and the hole's, off the boss (`Apart`), so the segment runs along the
  boss's rim inside the top. A one-segment rim's far end is the site
  itself, which the reduction pairs with the partner's vertex, so the
  pairing that means "one curve" held for every pair of closed edges
  (`one_segment`): both germs went `OnEdge`, the boss's rim and the
  hole's, and the join read both section loops on one side of the
  partner, `SectionLoopMixed`.
- **A touch with no vertex there.** With the boss's vertex elsewhere,
  the one-segment form has no vertex at the tangent point, where the
  two-arc form's hole has its second one (azimuth π). The rims' walls
  are tangent cylinders, so each rim is covered against the other's
  wall (`boolean/reduce.rs` `curved_face_arm`'s one-sided cover); its
  clearance reads `Zero`, the covered arm asked the interior question
  only of an edge ON the carrier (`interior`), deferred the pair, and
  `settle_deferred` found nothing split it: `CurvedPierceUnsupported`.
  The same holds in both forms wherever neither rim has a vertex at the
  tangent point, inside or outside.

## What builds it

- `boolean/sectors.rs` `closed_reach` reads a closed edge's midpoint
  where its two-arc form's far vertex would be, against each planar
  face around the partner's site whose plane holds it: inside one
  (`Face`, that face), on one's boundary (`Boundary`) or outside
  (`Apart`). `germ_loci` takes that reading in the far end's place, so
  the boss's rim runs `OnEdge` in the top's interior and the hole's rim
  is only tangent. Two closed edges stay one segment (`one_segment`)
  unless a midpoint reads inside or outside the partner's face: the
  concentric rims, the coaxial stack and the puck read as before.
- `boolean/join.rs` `closes_at_its_site` takes the one-site record of a
  closed edge against a vertex of a planar face's boundary
  (`SiteCell::Other`, which now names its face): the joiner walls the
  rim off in the top from the shared vertex, as `ChordJoiner::join`
  does any section loop of one face loop.
- `boolean/reduce.rs` `interior` asks the boundary pass of a covered
  edge off the carrier too: a meeting with the face's boundary is a
  curve × curve reading (`carrier_cross::boundary_crossing`), and where
  the edge's covered parent and the face leave the meeting to opposite
  sides of it (`sheets_part`, each face's `N × τ` at the point), the two
  tangent sheets meet only there, so the sweep splits both rims at the
  touch, as a vertex there would. Where they leave it to one side, the
  sheets run on together through both faces' interiors, and a split
  there builds a body tier 3′ refuses (`face_face_touch.rs`
  `a_nested_disc_with_its_vertices_off_the_line_refuses_its_mid_arc_touch`,
  measured: its difference read a face–face `Tangent` record across a
  crescent of walls, `ContactContradicted`); that pair keeps the
  deferral and its typed refusal. An edge off the carrier still has no
  `Clear` certificate (`Interior::Unseen`).

## What it measures

`crates/sweep/tests/a_closed_conic_in_the_partners_face.rs`
`a_boss_over_a_hole_by_pose` now holds both forms to one cell per pose,
16 poses, at the default ε, 1e-6 and 1e-12;
`a_boss_over_a_hole_tangent_to_it_turned` holds the 12 tangent poses
turned off the axes (`torax_pose`). Every tangent-outside pose — the
vertices there, `circle()`'s phase (the boss's vertex there), the
hole's there, the boss's at 1 and neither there — builds `SOUND` in
one shell in every op and both orders, through and pocketed. Every
tangent-inside pose, `neither there` included, refuses the union
`TangentCrescentUnsupported` and builds ∩ (empty) and both
differences, in both forms
(`a-union-across-a-tangent-crescent-refuses-unbuilt`).

## Its class

- **A closed edge's far end read as its course.** Swept over
  `boolean/sectors.rs` for readings of a germ edge's far end (`touch(`,
  `.far`): `germ_loci` (fixed), `tangent_face` (a closed edge has no
  far end there, and it answers none), `runs_in` (reads the edge's
  midpoint already), `germ_locus` (the one-sided reading: a closed edge
  On at a vertex-on-face site reads its own site as `Face`, which is
  right where the edge lies in the face, as on a boss standing on a
  plate; no pose was found where a closed edge is only tangent to the
  face at its vertex and a germ runs along it). Not swept: other
  modules' `half_edge_end` reads (26 in `topo/src/boolean`), which walk
  topology rather than read an edge's course.
- **A tangency the crossing layer leaves unsettled at the face's
  boundary.** The covered arm is fixed (`interior`). The uncovered arm
  (`wall_crossing`'s tangency, `SpanVerdict::Unsettled`) is filed:
  `a-cone-boss-over-a-hole-tangent-at-its-rim-refuses-in-every-pose`.
