---
id: tier-3-prime-reads-no-side-between-coincident-curved-edges
kind: issue
title: Tier 3′ certifies coincident curved edges by their bounds and never reads their wedges
status: open
opened: 2026-10-21
priority: P3
cost: M
---


Found by `tier-3-prime-reads-no-side-between-coincident-edges-on-one-line`'s
lane (branch `restread/coincident-wedges`), by reading.

## What

Tier 3′ now reads the material wedges of two coincident line edges
wherever pass 5 certifies their overlap (`census.rs`
`ee_collinear_lane`, `ee_overlap_sides`, through `line_wedges`). Pass
5′ (`census.rs` `sweep_curved_edge_edge`) certifies an overlap of two
curved edges along one curve by its records or a declared face pair
(`Declared::sides_face_backed`) and reads no side. `line_wedges` reads
line edges only (`edge_geo`, over `line_edges`), so it cannot be handed
the pair.

D1 (`crates/topo/README.md` tier 3) has the material sectors about one
curve disjoint as about one line. Two coincident arcs whose wedges
overlap pass tier 3, which reads one edge at a time, and tier 3′.

Known instance: PR 4622's curved keel slit, whose naive revert would
overlap its two arcs' wedges as the line slit's does
(`work/topo/revert-re-pairs-the-sheets-on-a-line-where-coincident-edges-meet.md`,
"Coincident curved edges are outside both readers"). Unmeasured.

## Owed

Read the two wedges at a point of the shared arc (each edge's two face
sheets square to the curve's tangent there), as `line_wedges` reads a
line's, and refuse an overlap `CoincidentWedgesOverlap` or its
undecided twin. A pinned witness: the curved keel slit's cavity grafted
reversed with no re-pairing
(`topo::test_support::grafted_reverted_unrefused`).
