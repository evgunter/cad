---
id: the-duplicate-button-is-live-over-its-doors-value-refusals
kind: issue
title: Commit duplicate is drawn live over add_duplicate's value refusals (Stale, NotLanded, NoValue, NotOneBody)
status: open
opened: 2026-09-28
priority: P3
cost: M
refs: [the-mirror-class-is-unswept-outside-the-properties-pane]
---


Found by the census in `the-mirror-class-is-unswept-outside-the-properties-pane`
(VNEWS), at merge base `f4e9aa68b`.

Commit duplicate (`crates/viewer/src/pane/create.rs`, the
`tool_commit_row` call ~:1496) is live whenever its seat is filled.
Past its kind gate (the sibling row
`the-creation-tools-commit-buttons-are-live-over-a-pick-their-door-refuses-by-kind`),
`DocSession::add_duplicate` (`crates/viewer/src/session.rs`) refuses
`Refusal::Duplicate(..)` at admission on the landed VALUE:

- `DuplicateFault::NotLanded` when nothing has landed;
- `DuplicateFault::Stale` whenever `busy()`. That is every click while an
  evaluation runs, and it persists after a cancelled one (busy, not
  running) until a re-evaluate;
- `NoValue`, `NotOneBody` (a transform of a pattern), `Unmeasured`,
  `NoExtent` from `combine::duplicate_step`.

The button reads none of them, and every one is a value the chrome can
reach: `landed_pair()`, `busy()` and `combine::duplicate_step` are all
available to the pane. The fix is a `DocSession` method answering what
`add_duplicate` would refuse the held seat with, read by the button the
way `slot_unit_refusal` is read by the unit picker. `duplicate_step`
tessellates twice, so a gate on it wants its answer cached per landed
generation rather than recomputed per frame; that caching is why this is
M rather than E.
