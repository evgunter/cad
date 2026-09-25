---
id: an-unassigned-id-under-a-refused-ray-is-unsaid
kind: issue
title: An id the drawn index never assigned goes unsaid on a cursor where the ray path refuses
status: open
opened: 2026-09-25
priority: P3
cost: D
---

Filed by `vnews/an-unnamed-id-is-not-nothing`, which closed
`an-id-the-index-cannot-name-is-announced-as-the-id-buffer-naming-nothing`.
This is what that fix left.

## The finding

`crates/viewer/src/idpass.rs`'s `disagreement` now reads the id
side as an `IdAnswer`, and an `IdAnswer::Unassigned` (an id the drawn
index never assigned, which is what a corrupt readback looks like) is
a disagreement against any ray ANSWER. But the function returns no
verdict before it reads the id at all when the ray path REFUSED
(`let Ok(from_ray) = from_ray else { return None }`). That is right for
a name (a refused path made no claim to contradict) and it leaves
out an unassigned id. An unassigned id is evidence about the device and
needs no ray to contradict it.

So on a cursor where the ray refuses, a corrupt readback is not said.
The frame says the refusal (`frame::pick_refusal`, by the pick path or
by `pane/viewport.rs`'s `cursor_news`), and nothing says the id.

## Why it was not fixed there

`cursor_news` returns one `Message`, and on a frame where the pick
path skipped the ray, that slot already carries the refusal. Saying
both needs either two notices from `cursor_news` or a sentence that is
not a `Disagreement`, because `Disagreement`'s shape is two answers.
Both are a change to what the comparison publishes. The residue is
narrow: the ray refuses on a camera fault and on the hit test's bug
arms, not on ordinary cursors, so an operator's issue #1097 §4 sweep
still sees the id everywhere the ray answers.

## Fence

`crates/viewer/src/idpass.rs` (CHROME, VGEOM) and `cursor_news` in
`crates/viewer/src/pane/viewport.rs`. Nearest sibling:
`rank-one-discards-the-frames-other-news`.
