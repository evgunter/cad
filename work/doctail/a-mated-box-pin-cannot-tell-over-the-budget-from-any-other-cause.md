---
id: a-mated-box-pin-cannot-tell-over-the-budget-from-any-other-cause
kind: issue
title: The mated-box tangency pin cannot tell over-the-budget from any other cause: swept_along drops the tier's OverTheBudget reason
status: open
opened: 2026-11-23
priority: P3
cost: E
---


Found by the delta review of PR 7396 (2026-11-23), a MINOR carried over from its first review and not yet fixed.

## What

`coincide::replay::swept_along` (`crates/editor-core/src/coincide/replay.rs`) returns a bool and drops the symbolic tier's `OverTheBudget` reason, so the recourse the door reports comes from the margin discharge that follows. The pin on `intent_s6_e_traced_on_a_face`'s mated-box row asserts `["Unproven", "replay: Numeric", "recourse: ReadTheAxis"]`, which stays red for any cause. Its comment, and `work/coincide/a-heading-through-a-chart-on-a-mated-copy-outgrows-the-doors-budget.md`, both claim a budget cause that nothing asserts.

Two doc NITs from the same review:
- `revalue_corpus.rs`'s boss_union parting doc (~:580) should say its pin moved 3 sooner too (2399 → 2396);
- `the_door_proves_two_thirds_of_the_composed_dies_joints` should say its 13 unproven joints are `Numeric/Budget`.

## Done when

`swept_along` carries its reason out (an enum, or a `Result` holding the tier's refusal) into what the door reports, and the mated-box pin asserts the budget cause specifically, so it reds for any other cause and flips when the coincide budget row lands.
