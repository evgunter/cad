---
id: a-join-onto-an-op-minted-edge-records-no-row
kind: issue
title: A boolean's join of an input edge onto an edge the op minted, on a class reading, records no row
status: open
opened: 2026-11-04
priority: P3
cost: H
---


## What

`crates/topo/src/boolean/ops.rs`, `join_rows` and `join_input_edge`: a
class-Zero row names its two edges as the input edges they lie on
(`RowCell::Input`, the only cell `editor-core`'s `coincide::name_rows`
names for a node). An edge the op minted after the reduction (a zip's
section edge) is in no operand clone's `Incidence`, so
`join_input_edge` reads `None` and `join_rows` writes no row for a join
that takes it.

Where both edges are minted they are pieces of one section curve, one
construction, and no row is owed. Where one is an operand's edge and
the other a section edge on one iso family through a regular vertex,
the join decided from values that the operand's edge continues the
section, and the `unproven-coincidence` lint does not see it.

No instrumented run of the topo or sweep `ci` profile reaches it
(pcert E3: every boolean iso-arm join joined two pieces of one input
edge). A row for it needs a spelling of a minted edge in input cells
(the face pair it was minted between, as `spell` widens a minted
vertex), which `Relation::SameOriented` over two faces would misstate.

## Done when

Such a join records a row the lint can name, or is shown unreachable.
