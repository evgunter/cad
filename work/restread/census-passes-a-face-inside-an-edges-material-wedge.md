---
id: census-passes-a-face-inside-an-edges-material-wedge
kind: issue
title: Tiers 3 and 3′ pass a face lying inside an edge's material wedge
status: parked
opened: 2026-10-12
priority: P2
cost: M
blocked_on: [4795]
---

Found by the keel design fork (`[ev]` PR 4795), measured on main
`360cf55eb2`.

## What

The boolean's wall-to-wall trench body of
`work/cleave/a-boolean-builds-a-reflex-edge-lying-inside-a-face.md`
puts the floor face through the keel edge's material wedge, and the
census reads nothing on it: tiers 3 and 3′ pass with no records. The
census skips every structurally adjacent pair (`sweep_vertex_edge`,
`sweep_edge_face` and the edge-edge sweep in `crates/topo/src/census.rs`
skip a shared vertex or shared face), and its only side test is for
`EdgeEdgeCross`. Compare `a_guest_seated_into_a_reflex_edge_is_not_a_rest`
in `census.rs`, which already knows a reflex edge's material reaches
across a plane. Not traced further.

## Fix

Under PR 4795's clause "no face passes through an edge's material
wedge", tier 3 (or 3′) refuses a face whose interior lies inside an
adjacent edge's wedge. Pin the boolean's three trench bodies (both ends
closed, one open, wall to wall) red until the boolean row lands.

## Note from CLEAVE (2026-10-14): the radial-edges fork on PR 4795

Both designers ask for one general check: **touching cells' material
lies on opposite sides of each other**, inside one solid as well as
between two. Concretely:
- a face lies outside an edge's wedge (this row);
- coincident distinct edges on one line have disjoint material
  sectors;
- a face lies outside a vertex's material cone.

Today the side-reading touch analysis runs only between two solids,
and inside one solid an edge-on-face record is confirmed by position
alone. A reverted slit body is the coincident-edge case: see topo's
`reverting-a-slit-body-overlaps-its-coincident-edges-wedges-and-passes-every-tier`.
The D1 sentence on 4795 names this check, so this row's scope grows
to cover it if Ev ratifies.

## Measured witnesses (topo row, 2026-10-14)

Each passes tiers 3 and 3′ on `origin/main` `ebf6c05239`, at rest:
- **Coincident edges, overlapping.** `subtract` of the slab
  `[−1,5]×[−2,4]×[0,1.5]` less the split's one-end slit body (the
  `slit_body` fixture of `crates/topo/tests/a_doubled_line_does_not_revert.rs`):
  one solid, volume 46.1875 (right), its floor carrying the keel as two
  edges each bounding about 286.7°. The same with the slab ending at
  `x = 3`, which cuts the slit (30.1125). ∖ now refuses both
  (`BooleanError::Revert`), so neither reaches the gate today; the
  check is what would catch the next door that writes one.
- **A face inside an edge's wedge.** ∖ of a glued tip, the tool's
  records carried: topo's
  `subtracting-a-glued-tip-reverts-its-edge-on-face-line-into-the-rider-form`.
