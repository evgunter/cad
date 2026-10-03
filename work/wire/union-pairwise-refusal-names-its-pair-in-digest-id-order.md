---
id: union-pairwise-refusal-names-its-pair-in-digest-id-order
kind: issue
title: a union's pairwise contact judgement names the refusing pair in id order, so which pair is named moves between ε rows
status: open
opened: 2026-10-02
---


## What

`judge_pairwise_contact` (`crates/editor-core/src/eval/wire.rs` ~3083)
visits a union's member pairs "in ascending node-id order, the lesser
id as operand A", and returns at the first pair that refuses. That
contract is the one DM4 states for the VERDICT: the verdict does not
depend on member order, and does not. But when several pairs refuse,
WHICH pair the `UndeclaredCoincidence` names also follows id order.
Since #3594 an id is a digest seeded through ε, so the pair named, and
the order a user meets the refusals in while repairing them one at a
time, moves between ε rows and has no relation to the document.

`docm7_union_declare.rs` (~1078, "Each pair is spelled lower id first,
and refused in id order") pins that order as it stands, and computes
its expectation from the ids. So the row passes at every ε, but what it
pins is the digest order.

Found by PLACE's document-order sweep (PR 3882,
`work/place/document-order-is-read-off-node-id-comparison-since-ids-are-digests.md`).

## The question

The naming order is WIRE's to choose:
- the members' document positions (`Doc::positions`), which are as
  member-order-free as ids are, and stable across ε;
- or the id order, kept and stated as a deliberate tie-break.

If the order moves, the operand-A choice can move with it or stay on
ids. It is the verdict's contract and need not change. Either way, the
`docm7` pin is restated to match.
