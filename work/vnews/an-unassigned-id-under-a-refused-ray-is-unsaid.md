---
id: an-unassigned-id-under-a-refused-ray-is-unsaid
kind: issue
title: An id the drawn index never assigned goes unsaid on a cursor where the ray path refuses
status: closed
opened: 2026-09-25
priority: P3
cost: M
branch: vnews/every-id-is-said
closed: 2026-09-28
pr: 5182
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

What blocks it is the sentence, not the slot. `Disagreement` is two
ANSWERS, and it has no sentence for a refused ray beside an id. The
slot is not the obstacle. On a frame where the pick path asked the
ray, the pick loop pushes the refusal itself and `cursor_news`'s
answer is empty (`disagreement` returns `None`), so there is room. On a
frame where it did not ask (`!ray_asked`), `cursor_news` returns the
refusal, and a second notice could still ride beside it, because a
frame joins several notices into one line. Either way the fix is a
sentence for *the ray refused here, and the id buffer answered an id
this picture does not draw*, which is a change to what the comparison
publishes. The residue is
narrow: the ray refuses on a camera fault and on the hit test's bug
arms, not on ordinary cursors, so an operator's issue #1097 §4 sweep
still sees the id everywhere the ray answers.

## Fence

`crates/viewer/src/idpass.rs` (CHROME, VGEOM) and `cursor_news` in
`crates/viewer/src/pane/viewport.rs`. Nearest sibling:
`rank-one-discards-the-frames-other-news`.

## Closed

Fixed on `vnews/every-id-is-said`. `crates/viewer/src/idpass.rs`'s
comparison is now `compare`, and it returns an `IdNews`: the two
paths' `Disagreement`, or `IdNews::BesideRefusal { id }` when the ray
path refused and the id buffer answered an id no patch of this picture
draws (`IdAnswer::Undrawn`). That arm says *"id buffer at the cursor:
id N, which no patch of this picture draws"*, in `IdAnswer`'s own
words, and leaves the refusal to the path that raised it.

*Nothing*, a name, and an unnamed patch beside a refusal are still no
verdict. Only a ray answer could contradict the first two. The third
stays out because the refusal a ray meets over a drawn patch is the
hit test's unnamed-entity arm, which already carries the same naming
error, so saying the id too would say one fact twice.

`crates/viewer/src/pane/viewport.rs`'s `cursor_news` returns every
notice it has, in order: the refusal when the pick path did not say
it, then the id news. The frame joins them into one line, and that
line's subject is `Document`: evidence is on
`one-line-one-subject-loses-a-mixed-frames-expiry`. Tests:

- `an_unassigned_id_beside_a_refused_ray_is_said_as_that_id` and
  `nothing_beside_a_refused_ray_is_no_verdict` drive both frames
  through `cursor_news`.
- `idpass::tests::beside_a_refusal_an_unnamed_patch_is_no_verdict_and_an_undrawn_id_is_news`
  pins the unnamed arm. No document can plant that value.
