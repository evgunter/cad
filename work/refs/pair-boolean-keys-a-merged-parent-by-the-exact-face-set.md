---
id: pair-boolean-keys-a-merged-parent-by-the-exact-face-set
kind: issue
title: The pair boolean keys a merged parent by its exact operand-face set, where the union's link is transitive
status: open
opened: 2026-11-02
priority: P3
cost: M
---


## What

`name_boolean` (`crates/editor-core/src/names/emit_topo.rs`) keys a
merged parent by the exact set of operand faces its merge lists
(`parent_at`, "two merges that list the same operand faces hold one
parent between them"). Two merges whose sets overlap without being
equal, {P0, P1} and {P1, P2}, are two parents that share P1. The
union's link is transitive (N2 in `crates/editor-core/src/names/README.md`;
`emit_union`'s `Parents::of`), so the same two merges in a union are
one parent `Merged{P0, P1, P2}`: a subtract and a union of one geometry
would name the shared constituent's faces differently, and a seam
beside P1 would have two parents to cite.

Reachability is unverified. A candidate: a boss whose wall is the three
pieces of `circle_split(.., 3, ..)`, cut by two slots so one merge glues
pieces 0 and 1 on one side and another glues 1 and 2 on the other. The
first job is a probe that builds it as a subtract and reads whether two
merge groups share an operand face; if none can, say why and close.
