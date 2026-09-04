---
id: opoutcome-superseded-has-no-production-reader
kind: issue
title: OpOutcome::superseded has no production reader — a discarded free-move probe is silent in the GUI
status: review
opened: 2026-09-04
refs: [viewer-session-god-module-split, rank-one-discards-the-frames-other-news]
branch: view/superseded-reaches-the-user
---


Found by the whole-file read that opened
`viewer-session-god-module-split` (2026-09-04).

## What happens

`OpOutcome` carries four fields
(`crates/viewer/src/session/op.rs:633`, `superseded` at `:646`).
The application reads exactly one of them: `app.rs:800` takes
`.refusal` and nothing else. `committed` is read by 21 test files;
`previewed` likewise; **`superseded` has no reader in `src/` at all**
— its only observers are `crates/viewer/tests/assembly_display.rs:607`
and `crates/viewer/tests/assembly_walk.rs:212`.

It is set on the paths where a free-move probe is discarded — an undo
(`session.rs:1056`) and a commit (`session.rs:1418`), both through
`self.display.prune`. So the one thing
the field exists to report, that the user's in-flight probe was thrown
away by something else they did, reaches the tests and never reaches
the user.

## Why this is a finding and not a nit

D-level fail-loud is the project's standing posture, and this is a
value the session computes correctly, hands to the GUI, and the GUI
drops. The status line is right there and already has a vocabulary for
it (`frame::StatusUpdate`). Whether a supersession deserves the line,
a badge, or nothing is a real question — but "nothing, undocumented,
while the type still promises it" is the one answer that cannot be
right, because the promise is what makes the next reader trust it.

It also constrains any split that moves `OpOutcome`: `refused` is
private and `Default` is derived, so a test cannot construct a refusal
outcome and must go through `perform`. That is a good property and the
split must not lose it.

## Home

VIEW's: `crates/viewer/src/session.rs` and `app.rs`. Rides unit 1's
ground; not unit 1's fix, since deciding what a supersession is worth
is a chrome question rather than a module-boundary one.


## Citations re-pointed after the 1c split (VIEW orchestrator, 2026-09-04)

This file was written against the pre-split tree. The `file:line`
citations above are corrected in place; this note exists so a reader
who remembers the old ones can tell a correction from a claim change.
Nothing about the finding moved — `stale-file-citations-after-the-split`
is the general case, and this is VIEW's own half of it being paid.


## What landed (VIEW-6, branch `view/superseded-reaches-the-user`)

**A supersession is NEWS, and its home is the status line.**
`frame::supersession_notice` is the rule, stated at the type:
`OpOutcome::superseded` renders to one notice, and `perform_batch`
pushes it onto the frame's `notices` so `frame_status`'s rank 2 shows
it. It is not a badge — it happened on one frame and is true of nothing
after it; the standing fact it leaves behind is the instance drawn at
its landed placement, which the picture already says.

It had to go through the notices rather than onto the line: the
transition that supersedes is an **accepted** edit, so that frame's
`batch_status` answers `Clear` and a supersession assigned to the field
would be erased by its own cause, before anything painted.

`ViewerBehavior`'s missing `notices` field — the stated blocker on
`status-line-writers-bypass-the-ranking` — does not apply: the read
site is `perform_batch`, on `ViewerApp`, which owns `notices`. Nothing
was threaded.

**Correction to this item's own text.** "The user's in-flight probe was
thrown away" is wrong about the field. `DisplayState::prune` returns
only the instances whose **committed** placements it discarded; a
gesture in flight when the transition lands dies too and is
deliberately not in the list (`crates/viewer/tests/review_gui4_r1.rs`
pins that). The paths are therefore a mate, a delete or a redo landing
on an instance whose placement the user had already committed — not an
undo taken during a drag. The type now says so.

**Also corrected**: the field had seven observing test files, not two
(`instance_authoring`, `story_assembly`, `review_gui4_r1`,
`review_gui4_r2`, `assembly_display`, `assembly_walk`, and now
`frame_policy`). The `src/` claim — no production reader — was exact.

Residue, filed: `rank-one-discards-the-frames-other-news`.
