---
id: a-mate-read-at-a-transform-under-a-union-refuses-read-below-a-root
kind: issue
title: Following PlacedUnderTwoRoots' recourse — union the two transforms of a mated instance — refuses each mate ReadBelowARoot, where Ev ruled it should work
status: closed
opened: 2026-10-03
priority: P1
cost: M
design: true
parent: MSOLVE-13
pr: 3969
closed: 2026-10-03
---


Ruled a defect by Ev on `[ev]` PR 3695 (2026-10-01): "the refusal
behavior you describe sounds like a defect (it should be possible to
do that)". Traced by one of that fork's designers; read, not yet run.

## What

One instance `top` feeds two `Transform`s, `t1` and `t2`, and each is
mated to its own base. The gather refuses this document as
`PlacedUnderTwoRoots`, and its recourse reads "place it under one
root, or union the two" (`crates/editor-core/src/product.rs`). Follow
the second half and add `Union { t1, t2 }`:
- The union re-mints its members' names (`FromMember`), so the
  product's name table no longer holds `top`'s face names as `m1` and
  `m2` spell them.
- `m1` reads its face at `t1`. `assembly::resolve_face` finds the
  product table silent and asks the operand. `t1` spells the name but
  is no longer a root, so the at-rest gate refuses
  `RefusedRef::ReadBelowARoot` (`crates/editor-core/src/assembly.rs`).

So the recourse leads into a second refusal. Ev ruled that the end
state, two placed copies of a mated instance fused by a union, should
work.

## What it wants

A design question first: how should a mate read a face whose operand
sits beneath a root that re-mints names? Options include:
- resolving the mate's operand read through the union's `FromMember`
  provenance;
- reading the face at the operand and verifying the contact at the
  root's re-minted face;
- something else.

Whatever is chosen also has to answer two things:
- what the gather's recourse should say, both for an instance (a
  pattern, or a second instance) and for a body document (a union);
- whether `ReadBelowARoot` stays a refusal anywhere.

**First step:** a red row on the traced document.

## Closed

Fixed by MSOLVE-13 (PR 3969). The traced document now gates: the gate
reads each mate's face at its operand and carries it up the consumers
to the product. Through the union, the face is carried as its member's
name, and both contacts mint
(`crates/editor-core/tests/msolve13_read_at_operand.rs`, A1(a)).

The design questions are answered as follows:
- **How a mate reads a face beneath a re-minting root.** By the lift
  (`names::lift`, beside `verbatim_edge`). A placer on the route
  refuses `RefusedRef::MovedAbove { at, by }`.
- **What the recourse says.** It branches on what is placed twice
  (`product::PlacedTwice`). For a body: union the two to fuse them,
  or pattern it to keep the copies apart. For an instance: instantiate
  it again or pattern it to place it twice, then union the two to fuse
  them.
- **Whether `ReadBelowARoot` stays.** No. `MovedAbove` replaces it, and
  every row that pinned it still refuses, as `MovedAbove`, or as
  `Vanished { by }` for the empty-boolean row.
