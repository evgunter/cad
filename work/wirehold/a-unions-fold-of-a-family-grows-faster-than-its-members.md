---
id: a-unions-fold-of-a-family-grows-faster-than-its-members
kind: issue
title: A union reading a family whole costs superlinearly in its members, inside one node with no yield
status: open
opened: 2026-11-01
---


Found by `wire-members` (WIREHOLD, `a-pattern-count-has-no-upper-bound-at-the-loop-that-uses-it`) while sweeping eval/ for loops over a family's members.

## Finding

A `union` whose `Bodies::Family` argument reads a family whole (`crates/editor-core/src/eval/wire.rs`: `wire_combine` wire.rs:3359, its `whole` closure wire.rs:3380-3405 with the per-member `.project(index)` at wire.rs:3390, then `combine_members` wire.rs:3467 → `fold_members` wire.rs:3565, whose fold is `run.run_members(..)` at wire.rs:3646 → `crates/verbs/src/run.rs:327` `run_members` → `crates/topo/src/boolean/nary.rs:542` `run_members`) does all of its per-member work inside one node, so the evaluation's cancel token (checked between nodes, and since this PR before each member of a family) is never read while it runs.

And its cost grows faster than its member count. Measured (debug build, this box, a row of `N` unit blocks a pitch of 2 apart, `fixture::row`, then `Node::Union { members: Bodies::Family(row) }`):

| N | the row alone | row + union |
|---|---|---|
| 25 | 37 ms | 4.2 s |
| 50 | 65 ms | 19.7 s |
| 100 | 132 ms | 119 s |

So doubling N multiplies the union's time by 5-6, roughly N^2.5, for blocks that never touch. Where it goes was not profiled. Two candidates, both read but not measured:
- `whole` calls `read_table(value).project(index)` once per member, and `NameTable::project` (`crates/editor-core/src/names/table.rs:812`, via `reindex` table.rs:823) walks the whole family table each time: O(N × total rows), which is O(N²) in members.
- the fold's own steps (`fold_members`, `images_apart`'s (wire.rs:3523) per-member `(i..len)` scan, which is O(N²) `Arc::ptr_eq` and cheap per pair).

## What it needs

- Locate the cost (a profile at N = 100), and remove whatever is quadratic in N that the union does not need. The per-member projection could be one pass that splits the family table by body.
- Cancelation inside the fold is DESIGN's "in-op yield points" (Cancelation bullet, "a union folding a family's members among them"), and it reaches `topo`'s n-ary run. That part is not this row's to decide.
