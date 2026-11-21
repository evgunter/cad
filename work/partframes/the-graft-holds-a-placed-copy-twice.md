---
id: the-graft-holds-a-placed-copy-twice
kind: issue
title: A placed copy is held twice through an aggregate: its source and its moved image
status: open
opened: 2026-11-18
priority: P2
refs: [7079, 7099]
cost: M
---

Filed by the ONEHOME orchestrator from fork 288 (PR 7079). All three
designers named it.

Each aggregate holds a placed copy twice: the `⊔` value keeps its
`Pieces` (`crates/editor-core/src/eval/wire.rs:4740`) and also the
fused body, built from each source moved into the computing frame and
grafted. `distribute::assembled` and `product::gather_sources` do the
same. Since PR 7099 the three graft through one door
(`topo::boolean::graft_placed`), but the fused body still holds
moved images.

`[placement.frame]` now says "an aggregate places each piece with
its frame", and that a moved copy "may be kept as a cache, and answers
a reading only where it gives what the record read through the motion
gives". So the fused body is either the cache (derived from the
pieces, never read for a verdict where it differs), or it goes in
stage 2 (`a-body-records-its-parts-in-their-frames`), when the body
holds each piece under its frame. Decide which when stage 2 is
planned; until then, no reader should take a verdict from the fused
image that the pieces read through their motions would not give.

## Moved to PARTFRAMES (2026-11-21)

Moved from `work/onehome/` by `git mv`, id and body unchanged (Ev, PR 7282: stage 2 split off); it is decided when stage 2 is planned. Priced `M`.
