---
id: mekr-and-kvfs-remove-an-empty-loop-or-lone-vertex-another-record-names
kind: issue
title: mekr's EmptyRing and BothEmpty sites kill an Empty ring no plan proves unclaimed, and kvfs removes a lone vertex a second Empty loop may hold: DanglingTopology through Ok
status: open
opened: 2026-09-30
refs: [kvfs-kills-a-lone-vertex-and-loop-a-torn-half-edge-still-names]
---

## What

Found by the second pass of the walk-proofs unit (PR 3511), which
looked past the walked plans at every removal of a loop or vertex
that reads its emptiness from a boundary rather than a walk.

- `Body::mekr` at `MekrSite::EmptyRing` and `MekrSite::BothEmpty`
  (`crates/topo/src/euler_ring.rs`, `mekr_empty_ring`,
  `mekr_both_empty`, killed in `mekr_finish`) removes an `Empty` ring
  loop without proving that no half-edge claims it. PR 3511 gives
  `kvfs` exactly that proof (`Body::require_run_of` over an empty run,
  `RunExtent::Whole`); these two sites have none.
- `Body::kvfs` (`crates/topo/src/euler_kill.rs`) removes its lone
  vertex without proving that no second `Empty` loop holds it; its
  doc says so ("not re-checked here").

## Measured

Throwaway rows at PR 3511's head (dev, inside a surgery scope):

- `strutted()` with the strut killed from its root (an `Empty` ring),
  a bystander segment solid whose plus half is torn to claim that
  ring, then `mekr_chord(EmptyRing { target: seg.he_minus, ring })`:
  `Ok`, leaving the torn half naming the dead ring.
- A segment solid, a lone solid, and a third `mvfs` solid whose outer
  loop is torn `Empty` at the lone vertex, then `kvfs(lone)`: `Ok`,
  leaving that loop holding a dead vertex.

## The shape to give

`mekr_empty_ring` and `mekr_both_empty` call `require_run_of([], ring,
RunExtent::Whole, &[])`; `kvfs` proves no other loop is `Empty` at its
vertex; both refuse `LoopCycleBroken` naming the loop. Pin one row
each.
