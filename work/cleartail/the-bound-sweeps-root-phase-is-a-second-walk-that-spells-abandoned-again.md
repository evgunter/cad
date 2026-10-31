---
id: the-bound-sweeps-root-phase-is-a-second-walk-that-spells-abandoned-again
kind: issue
title: Sweep::run's root phase is a second, unshared walk over candidates that spells abandoned a second time, and Question::overflow only ever fires on the root level
status: open
opened: 2026-10-29
priority: P4
cost: E
refs: [the-bound-walk-and-min-separation-duplicate-one-frontier-loop]
---


## Finding
From the single FULL review of PR #5976 (LOOPS) at `bb8d6a4dcc`. The
orchestrator accepted it and filed it rather than widening LOOPS,
whose claim was a refactor with no behaviour change.

- **Q1, sure.** The early-exit `abandoned` is spelled twice in
  `crates/topo/src/clearance.rs`:
  - in `Sweep::run`'s root phase, as `(candidates.len() - k - 1) + frontier.len()`;
  - in `walk`'s `Break`, as `w.left + w.next.len()`.

  The two "Unreachable: counted rather than skipped" branches (in
  `walk`, and in `Bound::pair`) are a second pair of copies. The root
  phase is an unshared mini-walk over the candidates.
- **NOTE.** Admission keeps `final + left + next ≤ max_cell_pairs`
  true throughout a level, so the whole-level overflow check in
  `walk` can only fire on the root level. So `Question::overflow`, and
  the distinction between the two overflow accountings, only decides
  whether the root frontier fits. The review's sweep agrees: across
  `min_separation`, `abandoned > 0` occurred only at
  `max_cell_pairs = 0`. This was already true before LOOPS.

Folding the root phase into `walk` (the root frontier as level 0)
would answer both points: one spelling of `abandoned`, and `overflow`
reduced to the one place it can fire.

Also from the same review (Q1, unsure), for whoever takes this up:
the sweep's default budgets are built inline in editor-core's
`ClearanceConfig::default()`, while `Budgets::MIN_SEPARATION` is a
const in topo. There is no `Budgets::SWEEP`.
