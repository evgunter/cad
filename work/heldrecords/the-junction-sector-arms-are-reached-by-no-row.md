---
id: the-junction-sector-arms-are-reached-by-no-row
kind: issue
title: The Junction arms of the sector readers (junction_lump, junction_bound_side) are reached by no row, the elbow unions included
status: open
opened: 2026-11-23
---


Found by HELDRECORDS' elbow-seams fix pass (2026-11-23), on the top of
the stack (`heldrecords/elbow-seams` with `heldrecords/elbow-ends`
617e30f4d3 and origin/main merged), answering PR 7432's reviewer.

## Measured

Three mutants of `crates/topo/src/boolean/sectors.rs`, each switched on
alone, over every topo, sweep and editor-core test matching
`a_pipe_unions_with_its_elbow|mate7a|junction|g1_tube|elbow` (88 tests,
the slow ones included):

- `JLUMP_IN`: `junction_lump` answers `SideCode::In`;
- `JBOUND_NEVER_ON`: `junction_bound_side` reads a bound riding the rim
  `Out`;
- `REACH`: `junction_lump` panics on entry.

All 88 pass under each, so no row reaches `junction_lump` (whose
callers are `recl`'s edge–edge and v–v readers and `vtxfac`'s
vertex-on-face reader). The four outward elbows, the rod, the G1 tube
chain, the neutral-point poses and an elbow drawn with its rim vertices
`0.3` and `1.0` rad round from the pipe's
(`an_elbow_drawn_with_its_rim_vertices_off_the_pipes_unions`, which
builds) all union without a sector site whose flanking pair is glued a
junction.

## Owed

A pose whose rim site's sectors are the junction-glued pair (a vertex
of one body on the other's wall at the rim, or a v–v site whose
flanking faces are the two walls), pinned so each mutant goes red; or,
if no geometry reaches the arms, the argument why, and the arms
reduced to it.
