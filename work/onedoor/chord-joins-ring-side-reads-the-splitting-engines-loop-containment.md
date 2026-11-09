---
id: chord-joins-ring-side-reads-the-splitting-engines-loop-containment
kind: issue
title: chord_join's ring side reads the splitting engine's in-plane loop containment (point_in_loop, LoopContainment, PointInLoopError) from outside the engine
status: open
opened: 2026-11-09
priority: P3
cost: E
---


Found by `onecopy/boolean-standing` (PR 6576), whose ring-side pin raised
`chord_join.rs`'s and `chord_join/ring_side_rows.rs`'s `names-splitting`
pins (`scripts/gates/engine-leaves.counts`, which cites this row).

## What

`chord_join`'s `ring_side` reads a bystander ring's side of a run from
each vertex's in-plane containment: `splitting::containment`'s
`point_in_loop`, `LoopContainment` and `PointInLoopError`, its standing
`PointInLoopError::refusal` included. `vertex_side` maps one reading to
the ring's rung (`Rung::answers`), and `chord_join/ring_side_rows.rs`
pins it on the error's own arms. Both name the splitting engine from
outside it, which `[topo.homes]` (`crates/topo/README.md`) says is a
reading the engine hosts and the crate root should.

## What closing it takes

Move the in-plane loop containment (the reader, its answer and its
error with its standing) out of `splitting/` to the crate root, as
`[topo.homes]` asks of a reading that takes a body, and re-point
`chord_join` and its rows at the root. Then drop the two pins' rises
this row's citation carries.
