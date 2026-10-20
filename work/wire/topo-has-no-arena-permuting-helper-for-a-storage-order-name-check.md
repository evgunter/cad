---
id: topo-has-no-arena-permuting-helper-for-a-storage-order-name-check
kind: issue
title: topo has no helper that rebuilds a body with its arenas permuted, so no test can check that a name ignores storage order
status: open
priority: P3
cost: E
opened: 2026-10-20
---

Filed by WIRE's `wire-nameinv` lane (branch `wire/name-invariance-checks`).

## What

`crates/editor-core/tests/name_invariance.rs` checks names against seven
perturbations. An eighth, an operand whose arena storage is permuted (the same
body with other keys and slot order), would reach build, completion and
pairing order, which a name must not read (N4: "a candidate number is storage
order"). It is not built, because `topo` has no helper that permutes a
body's storage. `rg -n "relabel|permute|shuffle" crates/topo/src` finds
`iso.rs` (which compares bodies up to relabeling), but nothing that produces
a relabeled body.

## What it needs

The smallest helper that would serve, behind `test-support`:
`topo::test_support::permuted(body: &Body<T>, seed: u64) -> (Body<T>, KeyMap)`.
It would rebuild `body` by inserting its vertices, edges, half-edges,
loops, faces and geometry in a seeded permutation of each arena. It would
return the old-to-new key map, and `topo::iso` would confirm the copy is
isomorphic. editor-core would also need a door to evaluate a recipe with one
node's value replaced by the permuted body, so that its names are carried
through. That door is the larger half and may not be worth it. The
alternative is a debug-only evaluation option that permutes every boolean
result's arenas before naming.
