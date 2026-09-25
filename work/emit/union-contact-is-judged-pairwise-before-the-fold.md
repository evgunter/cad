---
id: union-contact-is-judged-pairwise-before-the-fold
kind: unit
title: Build DM4's pairwise contact rule: every touching member pair is judged as its own two-member union before the fold, and a declared contact is satisfied wherever the fold meets it
status: open
opened: 2026-09-25
priority: P1
cost: D
---


## What

Ev ruled on #3200 (2026-09-25, "sounds good!"). This unit builds the
rule as DM4 now states it (`crates/editor-core/REFERENCES.md`):
- **Pre-pass.** Before the fold, `wire_union` judges every pair of
  members whose closed bounding boxes meet as the two-member union
  `m ∪ n`, with the declarations sited at `m` and `n`.
- **Undeclared contact.** A touching pair with no declaration refuses
  `UndeclaredContact` in every member order. That includes a contact
  another member covers.
- **Declared contact.** A declared contact is satisfied wherever the
  fold meets it, and also when the fold never meets it.
- **Consumed face.** A declared pair whose face the fold consumed whole
  before the pair's step is satisfied, not `Vanished`. A face that
  survives only in pieces stays with GATHER's look-through row.
- **The fold.** The fold makes no contact judgement of its own.

## Acceptance

- Under the new rule, `row` and `rowids` refuse `UndeclaredContact` in
  all 24 orders. With (a,h) declared, they fuse in the orders that fuse
  today. Measure this; the #3200 body labels it as a prediction.
  Update `KNOWN_MIXED` and the rows to match.
- Add a covered-contact row: `{a, b, h}` with `(a, h)` undeclared refuses
  in all six orders. Declared, it is satisfied in the orders where b
  consumed the face whole.
- Add member-level box pruning so that pairs whose boxes are disjoint
  skip the pairwise boolean. Measure the cost on the corpus's largest
  union.

## Scope note

The pre-pass is on WIRE's ground (`eval/wire.rs`), so name the crossing
in the PR. A cheaper contact-only kernel door is optional, and belongs
to REACH or ZIP.

The pre-pass costs up to n(n−1)/2 two-member unions, bounded by box
pruning. If this becomes a measured performance problem, it is raised
to Ev to be revisited, not optimized around the rule (Ev on #3200).
