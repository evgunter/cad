---
id: split-vertex-pairs-orientation-flips-in-the-whole-orbit-strut-case
kind: issue
title: SplitNaming::vertex_pairs is documented and read as (above copy, below original), but the whole-orbit strut case mints the copy as the below end
status: open
opened: 2026-10-02
priority: P3
cost: E
refs: [split-halves-have-no-contact-records-so-no-pseudomanifold-self-check]
---


## What

`names/emit_topo.rs` reads `vertex_pairs`' `.1` as the original. In `splitting/insert.rs`'s whole-orbit strut case the copy is minted at the below end, so that pair would come out `(original, copy)`. Reproduce with a row before any fix; if it holds, either the producer or the doc/consumer is wrong.

Found by the TQUERY designer pair weighing `split-halves-have-no-contact-records-so-no-pseudomanifold-self-check` (2026-10-02); read from the tree, NOT run — measure first.
