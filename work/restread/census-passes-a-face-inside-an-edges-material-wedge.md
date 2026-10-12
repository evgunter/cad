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
