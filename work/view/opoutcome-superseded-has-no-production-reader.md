---
id: opoutcome-superseded-has-no-production-reader
kind: issue
title: OpOutcome::superseded has no production reader — a discarded free-move probe is silent in the GUI
status: review
opened: 2026-09-04
refs: [viewer-session-god-module-split, rank-one-discards-the-frames-other-news, prune-discards-the-fault-that-explains-the-supersession, prune-drops-a-hidden-instance-silently, frame-module-has-eight-concerns-and-no-holds-row, the-news-vocabulary-has-no-expiry]
branch: view/superseded-reaches-the-user
pr: 1872
---


Found by the whole-file read that opened
`viewer-session-god-module-split` (2026-09-04).

## What happens

`OpOutcome` carries four fields
(`crates/viewer/src/session/op.rs:634`, `superseded` at `:644`).
The application reads exactly one of them: `app.rs` `perform_batch`
takes `.refusal` and nothing else. `committed` is read by 21 test
files; `previewed` likewise; **`superseded` has no reader in `src/` at
all** — its only observers are seven test files
(`instance_authoring.rs:178`, `story_assembly.rs:492`, `:634`, `:693`,
`review_gui4_r1.rs:446`, `:816`, `review_gui4_r2.rs:506`,
`assembly_display.rs:607`, `assembly_walk.rs:212`).

It is set on the paths where a **committed** free-move placement is
discarded — an undo or redo (`session.rs:1062`) and a commit
(`session.rs:1424`), both through `self.display.prune`. So the one
thing the field exists to report, that a placement the user made by
hand was thrown away by something else they did, reaches the tests and
never reaches the user.

A gesture in flight when the transition lands dies too and is
deliberately NOT in this list (`review_gui4_r1.rs:816` pins it), so
this finding is about committed placements only.

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

**Corrected in place above**, per this file's own convention, so a
reader top to bottom does not meet the wrong claim first:

- "the user's **in-flight** probe was thrown away" — wrong about the
  field. `DisplayState::prune` returns only the instances whose
  COMMITTED placements it discarded; a gesture in flight when the
  transition lands dies too and is deliberately not in the list. The
  discarding paths are a mate, a fuse, a delete or a redo landing on an
  instance whose placement was already committed — **not** an undo
  taken during a drag.
- **two** observing test files — there are seven (nine call sites),
  now eight with `frame_policy`. The `src/` claim, no production
  reader, was exact and was the load-bearing one.
- `session.rs:1056`/`:1418` and `op.rs:633`/`:646` — off by a few
  lines each.

Residue and follow-on, filed with this PR:

- `rank-one-discards-the-frames-other-news` — a refusal in the same
  frame drops the notice, and this is the case where the dropped
  message reports something unrecoverable.
- `prune-discards-the-fault-that-explains-the-supersession` — the
  sentence the user gets names the instance but not the cause, because
  the typed value carrying the cause is thrown away one layer down.
- `prune-drops-a-hidden-instance-silently` — the same class as this
  item, in the same function, on the `hidden` set.
- `frame-module-has-eight-concerns-and-no-holds-row` — where a ninth
  belongs is unwritten.

`the-news-vocabulary-has-no-expiry` gained this notice as a named
instance: "true of nothing after this frame" is a lifetime nothing
implements.
