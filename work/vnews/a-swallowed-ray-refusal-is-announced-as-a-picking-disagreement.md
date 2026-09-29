---
id: a-swallowed-ray-refusal-is-announced-as-a-picking-disagreement
kind: issue
title: A hit-test refusal the viewport drops is announced as the two picking paths disagreeing
status: closed
opened: 2026-09-22
priority: P2
cost: D
closed: 2026-09-25
branch: vnews/ray-refusal-is-not-a-disagreement
pr: 3221
---



Filed by `vgeom/seam-refusals`' sweep for the class *a typed refusal
that is computed and then dropped on the floor*.

## The finding

`crates/viewer/src/idpass.rs`'s `disagreement` states its premise in
its own header: the two paths *"disagree when the id buffer names a
face outside the set, nothing where the ray named something, or
**something where the ray named nothing**"*. An empty `from_ray` is
therefore read as **the ray named nothing**.

`crates/viewer/src/pane/viewport.rs`'s `viewport_ui` builds `from_ray`
by calling `PickIndex::faces_under_cursor` and then `.ok()` — the
typed hit-test refusal is discarded, `unwrap_or_default()` turns it
into an empty `Vec`, and the comparison cannot tell it from a genuine
miss. So on a frame where the ray path REFUSED to answer and the id
buffer named a face, the status line says *"picking paths disagree at
the cursor: id buffer <name>, ray nothing"* — a claim about the model
that neither path made.

The refusal is not rare or synthetic: `faces_under_cursor` refuses on
an ambiguous hit and on a hit test the kernel declines, which are
exactly the cursors near a silhouette or a tie that the id pass is
most likely to answer with a face.

## Why it is this slate's

The deliverable is what the reader is TOLD. The value on screen is
unaffected — the id pass is advisory and `disagreement` reports and
never resolves (the same header) — so nothing is drawn wrongly; a
sentence is published that is not true. `a-derived-pick-index-failure-outshouts-its-cause`
is the nearest sibling here.

The swallow is at a line in `pane/viewport.rs`, which VGEOM also
claims; the fix is a question about what the comparison should be told,
not about a number.

## What a fix would have to decide

Whether a refused ray path means *do not compare at all* (the cheap
arm: no answer, no verdict, which is what `disagreement` already
returns for a stale or absent query) or *a third state the notice can
name*. `Disagreement`'s own shape assumes two answers.

## Closed: no verdict (`vnews/ray-refusal-is-not-a-disagreement`, 2026-09-25)

**A refused ray path is not compared.** `idpass::disagreement` now
takes the ray side as `Result<&[StableName], &PickError>`, which is
`faces_under_cursor`'s own answer, and a refusal returns no verdict.
That is the same outcome as a stale or absent id query. The viewport
passes the `Result` through and no longer collapses it. A ray path it
could not ask at all (no evaluation) is also no comparison now. Before,
it was an empty answer.

**Why not a third state.** A refused path makes no claim for the id
pass to contradict, so `Disagreement`'s two-answer shape is correct.
The refusal is the ray path's news, and it already has words
(`frame::pick_refusal`).

**Who says it, by construction.** The pick loop says a refusal
whenever it asks the ray at this cursor, because `hovered_for` seeds
through the same `PickIndex::seed` as `faces_under_cursor`. It does not
ask on a frame it skips. A `Hold` skips the hover, and `Hold` reads
only the cursor and the picture, so a camera move under a still cursor
produces a ray nobody on the pick path asked. The pane's
`cursor_news` therefore reads the pick loop's own record
(`ray_asked_at`, over the same `skips_the_ray` rule the loop skips by).
It says the refusal through `frame::pick_refusal` when the pick path
did not ask, and says nothing when it did. Each frame that computes a
refusal says it once, and never as a disagreement. The review found
that the first version of this fix claimed the hover spoke "on the same
frame". That is never the frame a verdict is possible on: the id answer
lands in the paint callback after `viewport_ui`, so the first frame
with a fresh answer is a `Hold`. That version left the camera-moved
refusal unsaid. Driven in
`a_ray_refusal_the_pick_path_did_not_ask_for_is_said_by_the_comparison`,
it returns `None` where the refusal is owed.

The row's
premise also needed correcting. A certified tie is not a refusal of
`faces_under_cursor`: it returns the tied set, and the comparison
already treats that as agreement. What refuses is the camera
(`PickError::Camera`) and the hit test's other arms (`NodeFailed`,
`EvaluationOfAnotherDocument`, `Unnamed`, …).

Tests: `pane::viewport::tests::a_refused_ray_path_is_no_verdict_against_a_named_face`
(the comparison) and
`a_ray_refusal_the_pick_path_did_not_ask_for_is_said_by_the_comparison`
(the wiring, across an `Ask` frame and a camera-moved `Hold` frame).

Filed from this unit:
`an-id-the-index-cannot-name-is-announced-as-the-id-buffer-naming-nothing`
(the same function's id-buffer side), and on VGEOM
`a-camera-move-under-a-still-cursor-is-read-as-the-same-question` (the
root cause above: the hover and the id answer go stale on a camera
move) and `marks-focus-drops-an-unnamed-patch-from-attribution`. The
`blend.rs` sites the review's callee sweep found were already filed as
`work/author/blend-swallows-the-edge-name-fault-the-index-calls-loud`.
