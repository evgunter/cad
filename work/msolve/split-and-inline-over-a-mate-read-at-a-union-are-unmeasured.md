---
id: split-and-inline-over-a-mate-read-at-a-union-are-unmeasured
kind: issue
title: Split and inline over a mate head read at a union are unmeasured since the member walk descends unions
status: parked
priority: P3
cost: M
parent: MSOLVE-13
opened: 2026-10-03
blocked_on: [3990]
---


Found by MSOLVE-13's sweep (PR 3969), which could not check it.

## What

MSOLVE-13 changed two things:
- The member walk now descends a `Union` at the member its
  `FromMember` qualifier names (`mate::member::walk`).
- A member is keyed by its instance and its placing chain
  (`mate::member::Member`). The operand is no longer part of the key.

`member_of` is also the admission rule for the refactor doors. Four
sites in `crates/editor-core/src/refactor.rs` read it:
- the split's frame-crossing check (`frame_survives`, now
  `read.chain.is_empty()`);
- the split's interface-crossing collector (`is_mate_edge_end`);
- the inline's `MateFrameCrosses` and `MatePairSplits` checks.

With this change, each of them admits a mate whose head is read at a
union. No row in the suite splits or inlines across such a mate.
Every editor-core row is green, so nothing that was measured moved.
The shape that was never measured is:
- a split whose cut takes the union but not the instance below it, or
  the reverse;
- an inline of a part whose mate reads through a union.

## What it wants

Rows that split and inline across a mate read at a union, each with
its verdict stated. Where the cut separates the union from its member,
either the refusal is typed or the head re-anchors as the AQ8 clause
says.

