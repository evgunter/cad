---
id: a-ring-vertex-minted-in-the-pierced-face-has-no-mints-row
kind: issue
title: The ring vertices a boolean mints in a pierced face and at a node are not in Mints, so a row naming one would publish a clone key as an input vertex
status: open
opened: 2026-11-01
---


Filed by TOUCHREC's `touchrec-inface` lane (branch
`touchrec/in-face-mint-keeps-its-pierce`), from that unit's sweep of the
boolean's vertex mint sites.

## What

`topo/src/boolean/mod.rs` `Mints` records two kinds of vertex the
reduction mints in an operand clone: on an edge (`Mints::push`, with
`EdgeSplit::by`) and inside a face where a held pierce stands
(`Mints::push_in_face`). `boolean/ops.rs` `input_end` reads every other
vertex as it stands, as a vertex of the op's input.

Two more sites mint a vertex in a clone and record nothing in `Mints`:

- `vtxfac::mint_ring` (called beside `vtxfac::ring_groups` in
  `boolean/mod.rs`) mints the ring vertex inside the pierced face, in
  the pierced operand's clone (`ring_vertex_in`);
- `vtxfac` `mint_node` (from `node::mint_nodes`) mints a ring vertex in
  each operand's wall at a node of two equal cylinders' sections.

A row naming either would read it as a vertex of the input, with a key
the input does not hold.

## Whether a row reaches one

Not established. `cite_rows` spells the decisions in `ledger.pending`.
The sweep's are written before either site mints
(`reduce::sweep_and_settle`, then `acc.finish()`), and a ring tie names
the piercing operand's vertices only (`ops.rs` `ledger`). The one
writer left is `ops.rs` `carried`'s minted meeting, which decides two
cells the op minted on one carried record's two cells. A pierced-side
ring vertex reaches it only where the pierced operand's own record
lands a second minted cell at the pierce point, so that operand touches
itself there. No such fixture is known.

## What it needs

A fixture whose pierced operand's lumps touch at the pierce point, or
an argument that no carried record lands there. If a row reaches it,
`Mints` keeps the ring vertex with its face, the way `push_in_face`
keeps a held one's, and with the piercing cell, the way `EdgeSplit::by`
keeps a split's crossing one.
