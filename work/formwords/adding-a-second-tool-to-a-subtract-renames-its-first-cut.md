---
id: adding-a-second-tool-to-a-subtract-renames-its-first-cut
kind: issue
title: Adding a second tool to a subtract renames the entities its first cut made
status: open
opened: 2026-11-23
priority: P2
cost: M
---

Found by the delta review of PR 7426 (a subtract's tool is a `Bodies`
seat).

## What happens

`wire_subtract` (`crates/editor-core/src/eval/wire.rs:3621`) lowers a
subtract by how many tools it has. With one tool it is one pair run
(`subtract_from`, `wire.rs:3705`), named the way a pair boolean is
named. With two or more it is the n-ary fold the union's members take
(`combine_members` → `fold_members`), named by the union's end pass
(`names::name_union`, `crates/editor-core/src/names/emit_union.rs:261`).
These are two naming schemes. So when an author adds a second tool to a
subtract that had one, the names of the entities the first cut made
change shape, even though that cut's geometry is the same. A selection
that read a face of the first hole by name then reads a name the node no
longer publishes.

## What would close it

One naming for a subtract whatever its tool count. Either name a lone
tool the way the fold names its tools, or have the fold name its body
kept and each tool's faces the way the pair run does. Then, for every
entity the first cut made, adding a tool keeps its name. A row: the plate
minus tool `t0`, then the same node with `[t0, t1]` where `t1` is
disjoint from `t0`; every name of a face of `t0`'s hole resolves alike in
both.
