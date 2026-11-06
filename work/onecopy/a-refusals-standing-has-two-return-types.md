---
id: a-refusals-standing-has-two-return-types
kind: issue
title: A refusal's standing beside its type has two return types: ladder::Refusal::rung and Kept (census's InteriorsUnread::kept)
status: closed
opened: 2026-11-05
priority: P3
cost: E
refs: [6536]
closed: 2026-11-06
pr: 6539
---


Found by PR 6536's delta review (ladder-climb-3).

## What

A type's "standing as a refusal" is now stated in two vocabularies.

- **`ladder::Refusal`, read with `rung`.** This one is used by:
  - `chart_singularity`'s `Unread` (`crates/geom-brep/src/chart_singularity.rs:320-323`, `self.refusal().rung(self, Rung::Blocked)`);
  - `fate.rs:1380`;
  - `PointInLoopError`, `PointInSolidError` and `ContainError::refusal`.
- **`Kept<Self>`.** This one is used by census's `InteriorsUnread::kept` (`crates/topo/src/census.rs:11451-11461`). PR 6536 chose it because census has no abort and no unsettled kind, so `Kept` is the whole of its classification.

Nothing says which one a new type should take. So the next type picks by whichever sibling its author reads first.

## What closing it takes

Decide the rule from the semantics. For example: a type with no abort and no unsettled kind returns `Kept`; one that can abort returns `Refusal`.

Then put that rule where the next author will meet it, in the `ladder` module docs or as a type-level conversion (`From<Kept<E>>` for the rung). Bring `chart_singularity`'s `Unread`, the exact sibling of `InteriorsUnread`, to the rule's side.
