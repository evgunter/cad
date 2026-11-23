---
id: a-split-of-a-family-has-no-halves-a-read-reaches
kind: issue
title: An indexed read of a split-of-a-family's half refuses WrongOperand: the read takes the split's whole Family value, not the half's members
status: closed
opened: 2026-11-09
priority: P0
cost: M
branch: formwords/split-of-family
pr: 7387
closed: 2026-11-23
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

## The evaluator panics on it: raised to P0 (2026-11-22)

Found by the review of FORMWORDS PR 7379, and reproduced. Box,
pattern ×2, a transform of the pattern (a placement reading it), then
`SessionOp::AddSplit { target: pattern }`. The body gate admits the
pattern and the commit lands. Because a placement reads the target, the
split door also writes `Assemble { [split.0, split.1] }` (the
"above ⊔ below" in `crates/viewer/src/session/op.rs`). Evaluating that
node reaches `split_port` from `reads_projected` and hits
`unreachable!("a split evaluates to its two sides")`
(`crates/editor-core/src/eval/wire.rs`), because a split of a family
evaluates to `ValuePayload::Family`. It is a panic, not the
`WrongOperand` this row first described, and an ordinary split gesture
reaches it with no pick of a half. Rows:
`review_denotes_body::a_split_of_a_transformed_pattern_evaluates` and
`every_body_seat_over_every_family_form_evaluates_without_panicking`,
on branch `formwords/denotes-body-review`.

Claimed by FORMWORDS on 2026-11-22 by `git mv`, because REFS has no
orchestrator and a panic on a normal gesture is P0. Under D10
`[repetition]` the fix is the evaluator projecting a split's ports per
member over a Family. The `unreachable!` becomes a typed refusal for
any shape that is still genuinely unreachable.

## Closed (2026-11-23)

PR 7387. The panic is gone: `split_port` refuses `HalfOfAFamily { read, input, half }` typed, naming the read and the split, with the recourse "split one member (`xs[i]`) and read that split's half". What a split of a family means beyond that is not this row's question. Under the binder ruling (`[repetition]`, PR 7416), a one-body seat handed a family is refused at the door, so a split never sees a family. That is built by `a-family-read-at-a-body-seat-maps-implicitly`.
