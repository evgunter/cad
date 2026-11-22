---
id: denotes-body-enumerates-its-gaps-against-the-operand-door-and-misses-one
kind: issue
title: combine::denotes_body names two directions it differs from the operand door and there is a third
status: closed
opened: 2026-09-21
priority: P1
cost: D
pr: 7379
branch: formwords/denotes-body
closed: 2026-11-22
---


## Finding

Found by AUTH-1's correctness reviewer (AUTHOR, PR 2955, 2026-09-21),
confidence `sure`, and **demonstrated by running it** rather than
argued.

`combine::denotes_body` (`crates/viewer/src/combine.rs`) is a
NODE-KIND predicate; the evaluator's operand door, `body_operand`
(`crates/editor-core/src/eval/wire.rs`), is a VALUE predicate. The
doc on `denotes_body` says it "tracks, and is not equal to" the
operand door and names TWO directions in which it differs — the empty
boolean, and `Sweep`. **There is a third and it is unnamed:**
`Node::Transform` is shape-preserving over `Instances`
(`wire_transform` → `placeable_operand` → `Placeable::Instances`), so
a transform over a pattern is a `denotes_body` node whose VALUE is
several bodies.

The reviewer's probe: box → pattern ×2 → transform, pick a planar
instance face. `denotes_body` answers yes; the value is `Instances`;
the evaluator refuses
`WrongOperand { expected: "body", found: "instances" }`.

## Why it is filed here and why P1

**A predicate that enumerates its own gaps is making a claim, and an
incomplete enumeration is a false one.** Every caller that reads the
doc and concludes "two known gaps, neither of which is mine" is
reasoning from a list that is short. That is architecture that
entrenches as things are built on it — `work/README.md`'s P1 band —
and the entrenching has started: **AUTH-1 is the first caller to make
the predicate load-bearing as a GATE**, and it inherited the gap.
AUTH-1 fixes its own instance by testing the landed value instead
(PR 2955); the predicate and its doc are CHROME's ground and are
what this row carries.

## What a taker decides

Whether the list is completed, or the doc stops claiming to be a list
at all, or `denotes_body` is replaced at its gating callers by a value
test. The reviewer's own framing is the useful one: the two
predicates answer different questions, and a doc that says "tracks,
and is not equal to" is the only thing holding them together.

## Where else to look

Every caller of `denotes_body` — `NodeKindWanted::Body`'s arm of
`session::refuse::admits` is the one that makes it a gate, and the
combining seats are the population it was written for. A taker asks
of each whether a kind answer or a value answer is wanted.

## Closed (2026-11-22)

PR 7379. The probe no longer reproduces: the edit door's slot-kind check on `Doc::read_of_node` and D10 per-member evaluation closed it. `combine::denotes_body` is deleted, and its kind check lives once in `session::refuse::holds_body`, read by `admits` and `admits_read`. The doc states what the answer is, not a list of where it is wrong. The rows found in review are filed: the split-of-a-family panic (`a-split-of-a-family-has-no-halves-a-read-reaches`), the subtract by a family (closed by `[ev]` 7386), and implicit mapping itself (`a-family-read-at-a-body-seat-maps-implicitly`, ruled on `[ev]` 7409).
