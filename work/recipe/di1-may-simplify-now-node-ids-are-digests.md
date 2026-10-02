---
id: di1-may-simplify-now-node-ids-are-digests
kind: issue
title: DI1's minting-entry walk may be more than digest node ids need
status: open
opened: 2026-09-30
priority: P1
cost: M
design: true
refs: [layer3-recipenodeid-aliases-across-rewinds, sibling-branches-mint-one-node-id-for-different-nodes]
needs_ev: true
---



## What

`crates/editor-core/IDENTITY.md` DI1 was written for counter ids: two
inserts on sibling branches minted one id for two different nodes, so
a held id had to carry the history entry that minted it, found by
walking up until the document no longer `has_minted` it
(`History::entry`, `Doc::has_minted`), and every holder checks descent
before liveness. The build is still open, P0, as
`work/vseam/layer3-recipenodeid-aliases-across-rewinds.md`.

Node ids are now digests of the document's mint chain
(`crates/editor-core/src/mint.rs`, `Mint::insert`; branch
`emit/node-id-digest-mint`). Two sibling branches mint different ids
for different inserts, so a name carried across them resolves
`NodeGone(ForeignNode)` rather than another node's face
(`crates/editor-core/tests/asm_parent_held_names.rs`,
`sibling_versions_mint_two_node_ids_and_neither_resolves_the_others_names`).
The case DI1 still covers alone is the SAME insert on two branches
from one chain point: it mints one id on both (DI1: "the same insert
mints the same one"). Only minting edits extend the chain, so the two
branches may differ in everything else — a `SetParam` upstream, a
delete — and the one id then names two nodes that evaluate
differently.

## The question

Whether a holder still needs the minting entry, or whether the id plus
`Doc::has_minted` plus liveness now carries DI1's guarantee, which
would let the layer-3 build hold a bare id. What a history REPLACEMENT
(`Open`, `NewDocument`) needs is a separate half: a fresh document's
chain starts at zero, so the same first insert in two documents mints
one id, and `Doc::id` (DI3) is what tells those apart.

DI1 is ratified text, so a simpler rule is a revision for Ev, weighed
by the designers first. It should be decided before the P0 build
lands, since the build is what a simpler rule would save.
