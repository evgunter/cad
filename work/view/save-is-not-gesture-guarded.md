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

`open` (`session.rs:1070`) carries the guard. `save`
(`session.rs:1105`) does not, thirty-five lines later. So a save
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
`SessionOp::permitted_during_value_gesture` checked once in `perform`,
which a fortieth operation cannot be added without answering.

**This item is the row that table must state a value for.** The
gesture-as-data unit lands the table stating today's behaviour exactly,
including `Save`'s current answer, and this item stays open afterwards
as the question of whether that answer is right — a refactor that
quietly changed it would be a behaviour change smuggled through a
mechanical move, which `docs/prompts/implementer-discipline.md` §3
forbids in as many words.

## What VIEW-1b established (2026-09-04)

The table landed and records `Save` as **permitted** mid-gesture, which
is what the code did before it. Two things the unit turned up that
this item did not know:

- **The behaviour is pinned deliberately, not by omission.**
  `crates/viewer/tests/review_gui3_r2.rs:504` —
  `a_save_taken_mid_gesture_writes_the_committed_document_not_the_preview`
  — asserts it, on the argument that a save writes the committed
  history and a preview is not in it. So this was answered once, with a
  reason, and the answer is only invisible because it lived in a test
  rather than beside the code. The table is now where a reader finds
  it.
- **`Open` and `Save` may not be the same case at all.** `Open` refuses
  because it REPLACES the document the drag previews against; `Save`
  does not. The asymmetry this item was filed on is therefore not
  obviously a defect, and the question narrows to whether writing a
  document while a gesture is mid-flight is sound — which the pinned
  row already argues it is.

**What stays open** is thinner than what was filed: whether the
committed-history argument covers every writer (a save-as into a new
directory rebinds the resolver, which is not the same as writing the
current document), and whether the tree should say anything at all
while a save happens mid-drag. If neither survives scrutiny, this item
closes as answered rather than fixed.

## Home

VIEW's: `crates/viewer/src/session.rs`, and unit 1's ground.


## Citations re-pointed, and one CLAIM retired (VIEW orchestrator, 2026-09-04)

`file:line` citations above are corrected in place against the
post-split tree, so a reader who remembers the old numbers can tell a
re-point from a claim change.

**One of them was not a re-point.** "What happens" describes 23
`if self.gesture.is_some()` guards at 23 call sites, and says `open`
carries one where `save` does not. **That tree is gone.** VIEW-1b
replaced the guards with one exhaustive table
(`SessionOp::permitted_during_value_gesture`,
`crates/viewer/src/session/op.rs:586`) checked once in `perform`; two
`self.gesture.is_some()` reads survive in `session.rs` and neither is
a dispatch guard. `open`'s asymmetry is now stated in its doc comment
(`session.rs:1065-1069`) and its answer is a row in the table, not a
line in the function.

So the numbers above are right and the mechanism sentence around them
is not. That is the expensive half of
`stale-file-citations-after-the-split` — the CLAIM going stale rather
than the number — found in this program's own file while paying the
cheap half, which is the second time in two days that this program's
prose has outrun its tree. Read "What happens" as the 2026-09-04
finding that opened the item; **"What VIEW-1b established" below is the
current state**, and what is still open is stated there and only there.
