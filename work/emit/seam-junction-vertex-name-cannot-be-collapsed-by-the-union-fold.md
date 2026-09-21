---
id: seam-junction-vertex-name-cannot-be-collapsed-by-the-union-fold
kind: issue
title: A seam JUNCTION vertex's multi-Seam name is a shape emit_union's collapse refuses
status: open
opened: 2026-09-15
priority: P0
cost: H
---


## What

`emit_topo`'s seam-vertex pass names a seam JUNCTION — the vertex where
k ≥ 2 seam LINES meet — with a path that is the sorted list of those
lines' `Seam` segments, so the name has k ≥ 2 segments and every one of
them is a `Seam` (`crates/editor-core/src/names/emit_topo.rs`, the
`seam_lines.len() >= 2` arm of `name_boolean_edges`'s vertex pass,
~1252).

`emit_union::collapse` accepts a `Seam` only as the HEAD segment. Its
tail loop admits exactly `Fragment`, and refuses every head segment
found in tail position — `Seam` included — as
`NamingError::Emission` with `FOREIGN` ("a union fold's table carries a
segment the boolean emitter does not mint",
`crates/editor-core/src/names/emit_union.rs`, ~271).

So the two halves of one emitter disagree about a name shape the first
half mints: the boolean pair emitter DOES mint it, and the union fold
says it does not.

## Evidence

Reached from an ordinary declared union, measured 2026-09-15 on
`origin/main` at `72c222f1b` with a temporary `eprintln!` at the tail
arm. The fixture that reached it, which is legal:

- four members — `a` = x∈(0,1), `b` = x∈(0.5,1.5), both y,z ∈(0,1),
  caps and y-walls declared flush between them; slabs `g` = x∈(0.3,0.4)
  and `h` = x∈(1.0,1.1), both y∈(-1,2), z∈(0.5,3.5), declared against
  nothing — in the order `[b, g, a, h]`.

Six refusals fired across that lane's whole fixture sweep, so more than
one order and more than one document reach it; the taker re-measures
rather than citing this count. The refused segment is a
`Seam { a: <FromA/FromB face>, b: <FromMember face> }` sitting at path
position 1 — the second segment of a junction vertex's name.

## Why it matters

Unlike the two refusals
`two-emitter-refusals-a-legal-declared-union-reaches` re-classifies,
this one is NOT a missing rule: the rule exists and is minted, and a
second door in the same crate refuses to read it. It is an emission bug
of the kind `Emission` is for, so its classification is honest — but the
defect it reports is real and is in WIRE's own two files. Either
`collapse`'s tail admits a `Seam` run (the junction name's shape), or
the junction mint spells the name some other way.

## Found by

The sweep for `two-emitter-refusals-a-legal-declared-union-reaches`
(WIRE lane `wire-e2`), which measured which `bug(...)` refusals a legal
declared union reaches. Out of that unit's fence: the unit
re-classifies refusals, and this one is correctly classified.
