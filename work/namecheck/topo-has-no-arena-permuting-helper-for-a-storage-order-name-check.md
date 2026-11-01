---
id: topo-has-no-arena-permuting-helper-for-a-storage-order-name-check
kind: issue
title: topo has no helper that rebuilds a body with its arenas permuted, so no test can check that a name ignores storage order
status: open
priority: P2
cost: M
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

## What storage order the other perturbations already vary

Perturbations 1 (member order), 2 (operand order) and 7 (authoring
history) vary storage order only where it follows the fold. A union or
intersect folded in another order builds its result's arenas in another
order, so its slots and keys differ between the two runs, and so do
those of every later boolean that reads it. Each operand, though, is
built by one construction (an extrude or revolve of one profile, or a
fold in the listed order), and that construction inserts its vertices,
edges, loops and faces in the same order every time. Perturbation 7
changes the mint ids, not the arenas. No test today reads the names of
one operand body stored two ways, with the order it is folded in held
fixed. That is what this perturbation adds, and N0 names it as a check.

## What it needs

The smallest helper that would serve, behind `test-support`:
`topo::test_support::permuted(body: &Body<T>, seed: u64) -> (Body<T>, KeyMap)`.
It rebuilds `body` by inserting its vertices, edges, half-edges, loops,
faces and geometry in a seeded permutation of each arena, returns the
old-to-new key map, and `topo::iso` confirms the copy is isomorphic.
editor-core then needs a test door that evaluates a recipe with one
node's value replaced by the permuted body, with that node's table
carried across the key map, so the names downstream of it are read
again.

## Moved to NAMECHECK (2026-10-20)

WIRE measured 69 budget points against 30. This row is one of N0's checks, so it moved with them by `git mv` (id and body unchanged) to the program that holds them. Signed (WIRE orchestrator).
