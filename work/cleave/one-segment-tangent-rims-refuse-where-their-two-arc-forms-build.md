---
id: one-segment-tangent-rims-refuse-where-their-two-arc-forms-build
kind: issue
title: A boss over a hole whose rims are tangent refuses in its one-segment form where the two-arc form builds
status: closed
opened: 2026-10-26
closed: 2026-10-27
priority: P1
cost: H
branch: cleave/one-segment-tangent-rims
pr: 5808
---

Found measuring
`a-boss-over-a-hole-tangent-inside-its-rim-at-its-vertex-unites-as-two-shells`.

## What it was

The boss `r = 0.3`, `z ∈ [0, 1]`, over the plate `[−1, 1]² × [−0.5, 0]`
holed through or pocketed by a circle `r = 0.1`, each rim a one-segment
circle, or two arcs with vertices at the given azimuth and opposite.
Where the two-arc form built `SOUND` in every op, the one-segment form
refused:

| pose | one segment | two arcs |
|---|---|---|
| tangent outside (hole about `(0.4, 0)`), both vertices at the tangent point | ∪ `Join(SectionLoopMixed)`; ∩ and both differences built | every op `SOUND` |
| tangent outside, both vertices at azimuth 0 (`circle()`'s phase) | ∪ `Join(SingleSiteSectionLoop { count: 1 })` | every op `SOUND` |
| tangent outside, the boss's vertex at 1 | `CurvedPierceUnsupported` in every op | every op `SOUND` |

With no vertex at the tangent point at all, every op refused
`CurvedPierceUnsupported` in both forms, inside or outside.

## Why

- **A closed edge's far end is its own site.** At a vertex pair,
  `boolean/sectors.rs` `germ_loci` places a germ whose sectors both hold
  a bound read On by what the reduction recorded at each edge's far end
  (`touch`). A one-segment rim's far end is the site itself, which the
  reduction pairs with the partner's vertex, so every pair of closed
  edges read as one curve (`one_segment`): both germs went `OnEdge`, and
  the join read both section loops on one side of the partner,
  `SectionLoopMixed`. With only the boss's vertex at the point, the
  one-site loop met the hole's rim there (`boolean/join.rs` `site_cell`
  read `SiteCell::Other`), which no arm took.
- **A touch with no vertex there.** The rims' walls are tangent
  cylinders, so each rim is covered against the other's wall
  (`boolean/reduce.rs` `curved_face_arm`'s one-sided cover); the covered
  arm asked the boundary question only of an edge ON the carrier
  (`interior`), deferred the pair, and nothing split it. The two-arc
  form built where its second vertex (azimuth π) sat at the point.

## Closed

2026-10-27, PR 5808.

- `sectors.rs` `closed_reach` reads a closed circle's midpoint where its
  two-arc form's far vertex would be, from geometry rather than the
  records: against the planar faces around the partner's site
  (`on_planar_face`, shared with `runs_in`), inside one, on one's
  boundary within the band, or outside. Two closed edges take the
  reading only where both midpoints read and not both on the boundary;
  otherwise they stay one segment, as the concentric rims, the coaxial
  stack and the puck read. A curved partner, or a closed edge that is
  not a circle, is not read.
- `join.rs` `closes_at_its_site` takes a closed circle against a vertex
  of the boundary of a planar face it stands on from the other side
  (`stands_on`), the joiner minting its copy in that face as one chord
  from the vertex round to itself. A closed edge that is not a circle
  has no copy, and a circle lying flush in the face reaches a fate cut
  with no corner at the vertex, so both loops refuse
  `SingleSiteSectionLoop`.
- `reduce.rs` `interior` splits a covered rim off the carrier where it
  touches the face's boundary and the two tangent sheets leave the
  touch to opposite sides (`sheets_part`); where they run on together
  (`face_face_touch.rs`'s nested disc) the pair keeps its deferral and
  typed refusal.
- `sectors.rs` `runs_into` reads a kept edge tangent to a dropped
  sector's bound one order down, so a vertex pair at a touch records the
  rim fragment running from it into the dropped face as a fate edge
  (`recl.rs` `recl_sectors`); without it a rim split at two touches,
  in either form, left a fate cut with a slit.

Pinned by `crates/sweep/tests/a_closed_conic_in_the_partners_face.rs`:
`a_boss_over_a_hole_by_pose` (16 poses, one cell per pose for both
forms), `a_boss_over_a_hole_tangent_to_it_turned` (the 12 tangent poses
turned), `a_boss_over_two_holes_tangent_outside_with_no_vertex_at_either_touch`,
`a_rim_flush_in_its_partners_top_tangent_inside_builds_or_meets_a_pinned_limit`
and `one_segment_ellipse_rims_tangent_outside_refuse_the_union_typed`, at
the default ε, 1e-6 and 1e-12. Every tangent-outside pose builds `SOUND`
in every op; every tangent-inside pose refuses the union
`TangentCrescentUnsupported` and builds ∩ and both differences.

What stays open:

- the crescent union:
  `a-union-across-a-tangent-crescent-refuses-unbuilt` (P1);
- the uncovered tangency, a cone boss over a tangent hole:
  `a-cone-boss-over-a-hole-tangent-at-its-rim-refuses-in-every-pose` (P2);
- one-segment ellipse rims:
  `one-segment-ellipse-rims-tangent-at-a-point-refuse-where-circles-build` (P1);
- a one-segment cylinder through a plate, a hole tangent to its wall:
  `a-one-segment-cylinder-through-a-plate-tangent-to-a-hole-refuses-single-site` (P1);
- a closed rim flush in a partner face, tangent inside its boundary at
  a vertex, where the fate cut finds no corner (on `main` too):
  `a-flush-closed-rim-tangent-at-its-vertex-reaches-the-fate-cut-limit` (P1);
- a box along a fillet's line–arc junction (on `main` too):
  `a-box-along-a-fillets-line-arc-junction-reaches-classification-invariants` (P1);
- the inward direction spelled at five sites:
  `a-face-inward-direction-is-spelled-in-five-places` (P3).

## Its class

- **A closed edge's far end read as its course.** Swept over
  `boolean/sectors.rs` for readings of a germ edge's far end (`touch(`,
  `.far`): `germ_loci` (fixed), `tangent_face` (answers none for a closed
  edge), `runs_in` (reads the midpoint), `germ_locus` (the one-sided
  reading: a closed edge On at a vertex-on-face site reads its own site
  as `Face`, right where the edge lies in the face; no pose was found
  where a closed edge is only tangent there and a germ runs along it).
  Not swept: the 26 `half_edge_end` reads elsewhere in
  `topo/src/boolean`, which walk topology.
- **A tangency the crossing layer leaves unsettled at the face's
  boundary.** The covered arm splits (`interior`); the uncovered arm
  (`wall_crossing`'s tangency) is the cone-boss row above.
