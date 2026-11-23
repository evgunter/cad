---
id: an-id-the-index-cannot-name-is-announced-as-the-id-buffer-naming-nothing
kind: issue
title: An id the drawn index cannot name is announced as the id buffer naming nothing
status: closed
opened: 2026-09-25
priority: P2
cost: D
closed: 2026-09-25
branch: vnews/an-unnamed-id-is-not-nothing
pr: 3249
---



Filed by `vnews/ray-refusal-is-not-a-disagreement`'s sweep for the
class *a typed refusal computed and then dropped*, which closed
`a-swallowed-ray-refusal-is-announced-as-a-picking-disagreement`. This
is the same comparison's OTHER side.

## The finding

`crates/viewer/src/idpass.rs`, `disagreement`, builds the id buffer's
side as

```
index
    .name_of(id)
    .and_then(|name| name.as_ref().ok())
    .cloned()
```

`PickIndex::name_of` (`crates/viewer/src/pickindex.rs`) answers three
things for an id that is not `IdMap::NOTHING`: `Some(Ok(name))`,
`Some(Err(HitTestError))` for the unnamed-face bug arm, and `None` for
an id this index did not assign. The chain folds the last two into
`from_gpu: None`, which `Disagreement`'s `Display` renders as *"id
buffer nothing"*. Neither is what the id buffer said: it named a patch
id, and the index could not turn it into a name.

Two consequences, both a sentence that is not true:

- Against a ray path that named a face, the status line reads *"picking
  paths disagree at the cursor: id buffer nothing, ray <name>"*.
- Against a ray path that named nothing, the two AGREE (`from_gpu:
  None` against an empty set), so an id the index cannot name, over
  empty space by the ray's account, is not reported at all.

The `None` arm matters most. An id the drawn index never assigned is
what a corrupt readback looks like (the `R32Uint` clear fault issue
#1097 §4 tells an operator to sweep for), and `drawn_index` already
keeps a mismatched index out of the comparison. So on the one path
left, the diagnostic misstates the symptom it exists to report.

## What a fix would have to decide

The same question as the ray side, but the answer may differ. The
unnamed arm is a refusal (the naming layer's). The ray side's
refusal was declined as no verdict, partly because the hover path
already words it (`frame::pick_refusal`). Check whether anything
words this one. An UNASSIGNED id is not a refusal. It is evidence
about the device, so a third state the notice names (the id buffer
answered an id this picture does not draw) may be the true sentence
here, where it was not on the ray side.

Neighbour: `work/fit/id-readback-failure-reads-as-nothing-under-the-cursor`
is the readback-side sibling (a failed read stored as
`IdMap::NOTHING`). That row is about the channel word. This one is
about reading back a word that did arrive.

## Fence

`crates/viewer/src/idpass.rs` (CHROME, VGEOM), read-only reach into
`pickindex.rs`'s `name_of`. Testable headlessly: `disagreement` takes
the channel word, so an unassigned id is `serial << 32 | id` with an
`id` past the index's range.

## Closed: a third state the notice names (`vnews/an-unnamed-id-is-not-nothing`, 2026-09-25)

**The id side is an `IdAnswer`, not an optional name.**
`Disagreement::from_gpu` is `Nothing | Named | Unnamed { id, error } |
Unassigned { id }`, built by `IdAnswer::of`. `Unnamed` and
`Unassigned` agree with no ray answer, and the sentence names the id:
*"id buffer id N, which no patch of this picture draws"* and *"id
buffer patch N, which the index has no name for (<refusal>)"*.

**Why not no verdict, as on the ray side.** The ray side's refusal
already has words and someone to say them. This does not. The id
answer's only reader is `cursor_news` in `pane/viewport.rs`, so no
other surface can say an unassigned id. An unnamed patch is said
elsewhere only when the ray meets the same face, because then
`pick_face` refuses with `HitTestError::Unnamed`. That is a ray
refusal: it gets no verdict and is said once, as before. And
agreement is a shared name, which these do not have.

Tests: `pane::viewport::tests::an_unassigned_id_over_empty_space_is_said_as_that_id`
and `an_unassigned_id_against_a_named_face_is_said_as_that_id`, one
per false sentence. Both drive `cursor_news` and assert fixed text.

Filed from this unit: `an-unassigned-id-under-a-refused-ray-is-unsaid`
(a refused ray still returns no verdict before the id is read).
Also filed after review: `a-corrupt-id-on-a-hidden-roots-patch-reads-as-a-named-face`
(here), on FIT `name-of-answers-none-for-both-nothing-and-an-unassigned-id`
(the typed split belongs at `PickIndex::name_of`, and `IdAnswer`
collapses into it), and on EDIT `an-unnamed-patch-is-reported-as-a-hit-test`
(the forwarded refusal's *"hit test:"* prefix).
