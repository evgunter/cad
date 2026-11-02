---
id: the-declaration-read-routing-survives-the-declarations
kind: issue
title: The refusal routing still reads a declaration that would settle a question, though nothing declares
status: open
opened: 2026-10-30
priority: P3
cost: M
refs: [a-glued-pair-is-re-read-through-the-declared-rung-which-bridges-in-band]
---

Since CONTACTHOLD (PR 6079), a glued pair is the glue door's decision,
read through its record. No caller states a coincidence. The
refusal routing still carries the machinery for "a declaration of
`class` would settle this question", which nothing can now realize:

- `DeclarationRead::{Settles, Spent}`, `Settling`, and
  `Coincide::settled_by` (`crates/topo/src/boolean/refusal_routes.rs`);
- `GluedPairs::read` and `door_refusal`
  (`crates/topo/src/boolean/mod.rs`), and each site that asks them
  (`vtxfac.rs`, `recl.rs`, `reduce.rs`, `sectors.rs`, `node.rs`,
  `join.rs`, `slit.rs`, `insert.rs`, `fate.rs`, `splitting/classify.rs`).

`Settles` offers a declaration as recourse, and no API takes one.
`Spent(class)` reads a pair the door glued that still refuses; that
fact is real, but it is not a declaration. Its fixtures plant a class
in the index that the door's verified set does not hold.
`reduce.rs`'s `run` helper and `refusal_routes.rs`'s `every_read` are
examples, and that state is not reachable.

The machinery has 207 hits across 30 files (`grep -rn
"DeclarationRead\|Settling\|settled_by" crates/`). It includes rows
outside topo (`sweep`, `profile`, `editor-core`), so it was left out of
PR 6079's fix pass.

The same pass renames the `declared: &GluedPairs` parameter, which has
29 signatures and is used throughout `crates/topo/src/boolean/`, to
`glued`. It shares those sites with the machinery.

**Done when**: `DeclarationRead` is gone or reduced to what a glued
pair's refusal states, no recourse offers a declaration, the planted
fixtures are built through the door or retired, and the `declared`
parameter is renamed.
