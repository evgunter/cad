---
id: MSOLVE-2
kind: unit
title: The member chain: nested patterns and sibling distinctness at every level
status: open
opened: 2026-09-05
refs: [nested-pattern-mate-heads-refuse]
---


## Waits on

`MSOLVE-1`, which gives `Member` its operand and the walk its single
pattern level. This unit generalizes `Member.copy` to the chain of
structural indices the walk consumes (sibling distinctness at every
level, the loop-closure argument holding at each), loses `Copy` if it
must, re-keys `by_pair` and `edge_of`, and owes the loop-closure rows
for a nested member that nothing in the suite has. Ruled in by Ev on
PR 1731 (`nested-pattern-mate-heads-refuse`, parked on this unit).
Spec is written when MSOLVE-1 merges, against the walk as landed.

## A decision this unit takes with it (from MSOLVE-1's report)

`Node::Part { select: PartSelect::Instance(i) }` is a third
identity-transparent node (`crates/editor-core/src/node.rs`, the
variant's doc: "every name VERBATIM"). It moves no geometry, so
MSOLVE-1's correctness did not need it, but a mate reference could be
read at one and the walk refuses it `DanglingHead` today. Rule on it
here, with the chain: admit as a pose-neutral pass-through, or fence
with a sentence at `member_of`.

## Carried from MSOLVE-1's reviews

- `Placer::Pattern { node, i }` and `Member.copy` spell the pattern
  index twice (`mate/solve.rs`); when `copy` becomes the chain it IS
  the walk's chain filtered to patterns — collapse them.
- The walk runs three times per reference per solve (`member_of`,
  `head_of` for `by_pair`, `derived_offset` via `pair_left_factor`);
  carry the `Walk` in `by_pair`'s value.
- `mate/solve.rs` is 1300 lines with the member vocabulary as its
  first third; split `mate/member.rs` out when the chain lands.
