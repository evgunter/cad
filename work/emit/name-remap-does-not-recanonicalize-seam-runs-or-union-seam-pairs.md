---
id: name-remap-does-not-recanonicalize-seam-runs-or-union-seam-pairs
kind: issue
title: refactor's name remap re-sorts Merged and SideOf but not a junction's Seam run or a union's canonical Seam pair
status: open
opened: 2026-09-23
priority: P2
cost: E
---


## What

`refactor::remap_seg` rewrites every local node id in a name and then
re-sorts the two orders it knows are NAME order — a `Merged` set and a
`SideOf` partner vector — because "the rewrite may have changed it"
(`crates/editor-core/src/refactor.rs`, `remap_seg`'s `set` closure and
its `Fragment(SideOf)` arm). Two more orders in the vocabulary are
name order and are not re-established:

- a seam JUNCTION vertex's path is the SORTED run of its lines' `Seam`
  segments (`crates/editor-core/src/names/emit_topo.rs`, the
  `seam_lines.len() >= 2` arm of `name_boolean_vertices`), and a
  union's collapse re-sorts that run in member space
  (`crates/editor-core/src/names/emit_union.rs`, `collapse`'s `Seam`
  arm). `remap_derivation` maps each segment in place and keeps the
  run's old order.
- a union's `Seam { a, b }` has its two sides CANONICALIZED by name
  order (`emit_union.rs`, `seam_line`). `remap_seg`'s `Seam` arm maps
  `a` and `b` in place. (A pair boolean's `Seam` is positional — A side,
  B side — and is correct as it is.)

## Why it matters

If a remap can reorder two names, a remapped junction or union seam
name is not the name the emitter mints for the remapped document, so
it does not resolve there. Unmeasured: whether any caller's node map
is non-monotone. `remap_seg`'s own `Merged`/`SideOf` re-sort is written
on the premise that it can be.

## Found by

The segment-shape sweep for `seam-junction-vertex-name-cannot-be-collapsed-by-the-union-fold`.
