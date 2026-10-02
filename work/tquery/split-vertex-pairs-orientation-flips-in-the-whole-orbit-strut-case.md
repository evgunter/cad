---
id: split-vertex-pairs-orientation-flips-in-the-whole-orbit-strut-case
kind: issue
title: SplitNaming::vertex_pairs is documented and read as (above copy, below original), but the whole-orbit strut case mints the copy as the below end
status: closed
opened: 2026-10-02
priority: P3
cost: E
refs: [split-halves-have-no-contact-records-so-no-pseudomanifold-self-check]
closed: 2026-10-02
pr: 3820
---


## What

`names/emit_topo.rs` reads `vertex_pairs`' `.1` as the original. In `splitting/insert.rs`'s whole-orbit strut case the copy is minted at the below end, so that pair would come out `(original, copy)`. Reproduce with a row before any fix; if it holds, either the producer or the doc/consumer is wrong.

Found by the TQUERY designer pair weighing `split-halves-have-no-contact-records-so-no-pseudomanifold-self-check` (2026-10-02); read from the tree, NOT run — measure first.

## Closed

Reproduced: on the reflex prism the corner pair came out `(original, copy)`, and a recipe `Split` through that corner refused in naming (`Emission`: "on-plane vertex with neither a SplitEdge record nor an operand identity"). `splitting/finish.rs` now builds every pair as `(copy, at_vertex)` whichever end the copy took; pinned by `join_whole_orbit_rows::a_split_through_the_reflex_corner_whose_run_holds_the_whole_orbit` and `m4_pr3_names_rework::split_through_a_reflex_corner_names_its_copy_on_each_half`.
