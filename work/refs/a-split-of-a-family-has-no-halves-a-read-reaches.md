---
id: a-split-of-a-family-has-no-halves-a-read-reaches
kind: issue
title: An indexed read of a split-of-a-family's half refuses WrongOperand: the read takes the split's whole Family value, not the half's members
status: open
opened: 2026-11-09
priority: P2
cost: M
---



Found by the fix pass of PR 6593 (`viewer-boolean-tool-cannot-pick-a-read`).

**What.** A split whose target is a family (`Node::Split { target: pattern }`)
evaluates to `ValuePayload::Family`, a family of split values, which has
no output. Each of its halves is a variable evaluated once per member, so
the door admits one member of a half, `split.above[1]`
(`BodyRead::indexed(Operand::output(split, 0), [count(1)])`), and the
viewer's boolean tool now authors exactly that for a split of a family
picked in the tree, its half and member chosen. Evaluation then refuses it
on the reader: `reads_projected` (`crates/editor-core/src/eval/wire.rs`)
reads an indexed read's family through `value_of(results, family)` — the
SPLIT's whole value, a `Family` — and answers
`NodeErrorKind::WrongOperand { input: split, expected: "instances", found: "family" }`.
A plain read of a half of a split of a family fails the same way.

**Pinned** by
`combine_ops::a_split_of_a_family_reads_one_half_of_one_member_chosen_in_either_order`
(`crates/viewer/tests/combine_ops.rs`), whose last assertion is today's
refusal.

**Done when** a read of one half of a split of a family is that half's
members (an indexed read one member's half), the pinned row's last
assertion becomes the member's upper three quarters by volume, and
Python's `split[...]`/`IndexedRead` spelling of the same read evaluates.
The ground is the evaluator's operand door (`eval::wire`, owned by
heldrecords, tally and wirerest); it is on REFS's slate as the program
whose charter (indexed reads, DM3/DM4) it fits best.
