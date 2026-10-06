---
id: emit-topo-reads-a-fusion-chain-one-hop
kind: issue
title: emit_topo's seam-vertex pass reads vertex_merges kept → direct dead partners only, so a chain a→b→c never offers c the identity a carried
status: open
opened: 2026-10-06
priority: P2
cost: E
refs: [fused-into-names-vertices-not-live-in-the-result, 4116]
---

Found by the review of PR 4116 (ZIP, `zip/survivor-and-recourse`).
Inferred from the code; not measured on a scene.

## The finding

`names/emit_topo.rs`, in the seam-vertex pass (the function reading
`naming.vertex_merges` into `fused`), builds kept key → its DIRECT
dead partners:

```rust
for &(dead, kept) in &naming.vertex_merges {
    fused.entry(kept).or_default().push(dead);
}
```

A live vertex then looks for its operand identity among its own key
and `fused[&v]`. For a chain `(a, b), (b, c)` the live `c` sees only
`b`; `a`, which may be the only key an operand's table names (the
comment's own example: a B corner fused into an A crossing), is never
offered. `BooleanNaming::fused_into` already folds every hop
(`Fusions::survivor`); the pass could invert that instead.

## What is owed

Read every dead key onto its final survivor (invert `fused_into()`),
and pin a chain whose first dead key carries the only name. Check
first whether a two-hop chain reaches this pass on any corpus scene:
`fused-into-names-vertices-not-live-in-the-result` (FUSE) says some
survivors are not live, which this pass would then also miss.
