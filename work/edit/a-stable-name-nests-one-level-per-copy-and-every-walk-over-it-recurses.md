---
id: a-stable-name-nests-one-level-per-copy-and-every-walk-over-it-recurses
kind: issue
title: editor-core: a StableName nests one NameRef per pattern copy, part instance or merge, and its drop, Debug and descent walks recurse without bound
status: open
opened: 2026-09-30
priority: P1
cost: M
---


(EDIT, found by the recursion sweep of `edit/part-depth-bound`.)

## What

A `StableName` (`crates/editor-core/src/names/role.rs`) holds its
role path flat, but three segments hold a whole name inside:
`RoleSeg::Instance { of: NameRef }` (a pattern copy),
`RoleSeg::InPart { of: NameRef }` (an instantiated part's entity) and
`RoleSeg::Merged(Vec<StableName>)`. `NameRef` is an `Arc` of a name, so
a chain of K patterns, each over the one before, names its faces K
levels deep, and a part nested D documents deep adds D more. Nothing
bounds either.

Every walk over a name recurses once per level: the `Drop` of the last
`Arc`, `Debug`, `Hash`, `Ord`/`Eq` on names that share no `Arc`, serde,
and the descent walks `names::face_descends_from`, `seam_through`
(`names/seam_pair.rs`), `fold_descent` (`eval/wire.rs`),
`constituents_through_wrappers` (`names/merged.rs`) and
`resolve::walk_names`. The viewer renders a disagreeing pick with
`format!("{name} ({:?})", name.path)` (`viewer/src/idpass.rs`,
`name_and_path`), a `Debug` of the whole nesting.

Evaluation itself does not recurse on it (the peak below is flat), so
the crash lands after it: when the evaluation is dropped, or a name is
printed.

## Evidence

One document: a square extrude, then K `Node::Pattern`s, each over the
one before with `count: Expr::count(1)`. Evaluated, then the top
pattern's result formatted with `{:?}`, then the evaluation dropped;
the stack each step touched, measured by painting a 512 MiB thread's
stack:

| profile | K | evaluate | `Debug` | drop |
|---|---|---|---|---|
| release | 1 000 | 10 KiB | 799 KiB | 62 KiB |
| release | 10 000 | 10 KiB | 8 111 KiB | 765 KiB |
| dev | 1 000 | 243 KiB | 1 303 KiB | 270 KiB |

So about 80 bytes per level to drop in release and 270 in dev, and
0.8 to 1.3 KiB per level to print: a chain of about 3 500 patterns is
past the wasm32 build's 1 MiB stack in dev, 13 000 in release, and
printing one name crosses it near 700 levels in dev. A part chain at
`MAX_DEPTH` (1024 documents) puts 1024 `InPart` levels into every name
its top instance carries.

Not probed: whether a mate head naming a face that deep survives a
save and a load (the load door's `serde_json` recursion limit of 128
would refuse it typed on the way back in).

## What would close it

A `Drop` for the nesting that does not recurse, and walks that carry
their own stack (or a bound on nesting that every walk fits, refused
typed where a name is minted); `name_and_path` rendering the path
without `Debug`. A row that evaluates a chain of patterns deeper than
the smallest stack allows today, drops the evaluation and renders a
name, on the wasm32 build's stack.
