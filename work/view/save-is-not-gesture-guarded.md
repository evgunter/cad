---
id: save-is-not-gesture-guarded
kind: issue
title: Save is not gesture-guarded while Open is, one function apart, and nothing says whether that is a decision
status: open
opened: 2026-09-04
refs: [viewer-session-god-module-split, two-gestures-can-be-in-flight-together]
---


Found by the whole-file read that opened
`viewer-session-god-module-split` (2026-09-04).

## What happens

`session.rs` spells the mid-gesture guard `if self.gesture.is_some()`
**23 times** — one spelling, no variants — at `session.rs` lines 2100,
2177, 2223 (inside `perform`), then 2318, 2345, 2535, 2545, 2573,
2586, 2599, 2610, 2684, 2713, 2778, 2805, 2821, 2840, 2854, 2875,
2902, 2925, 2951, 2988.

`open` (`session.rs:2712`) carries the guard. `save`
(`session.rs:2750`) does not, thirty-eight lines later. So a save
during a slot drag persists the history while a scratch document is on
screen.

## Why it is filed as a question and not a patch

The set of gesture-safe operations is **not derivable from the code**.
It is 23 guards at 23 call sites and no table, so answering "is
operation X safe mid-gesture" means reading all 39 dispatch targets.
The unguarded set as it stands is `Select`, `Hover`,
`CancelEvaluation`, `Reevaluate`, the four `*FreeMove` ops,
`SetInstanceHidden` — every one of which reads as a deliberate
exemption — **and `Save`**, which does not.

That is exactly the shape where a patch is the wrong instrument: with
no table, adding a guard to `save` asserts a rule nobody has written
down, and leaving it asserts the opposite equally silently. Unit 1's
charter ("gesture-safety as data") answers it properly — one exhaustive
`SessionOp::gesture_safe` checked once in `perform`, which a fortieth
operation cannot be added without answering.

**This item is the row that table must state a value for.** The
gesture-as-data unit lands the table stating today's behaviour exactly,
including `Save`'s current answer, and this item stays open afterwards
as the question of whether that answer is right — a refactor that
quietly changed it would be a behaviour change smuggled through a
mechanical move, which `docs/prompts/implementer-discipline.md` §3
forbids in as many words.

## Home

VIEW's: `crates/viewer/src/session.rs`, and unit 1's ground.
