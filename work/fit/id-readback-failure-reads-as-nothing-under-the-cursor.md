---
id: id-readback-failure-reads-as-nothing-under-the-cursor
kind: issue
title: A failed id readback is reported as an empty cursor, and the chrome blames the picture for it
status: open
opened: 2026-09-16
priority: P1
cost: D
---



## Finding

Found by the sweep that closed
`datum-view-propagates-rather-than-refusing-by-name` and
`a-datum-the-view-cannot-scale-vanishes-without-a-word`, for the class
those two share: **a door that hands back a value it did not compute,
in a field shaped like one it did.**

`crates/viewer/src/gpu.rs`, `ViewportCallback::prepare`. The id pass
reads one pixel back and the read can fail:
`ViewportRenderer::read_id_at` answers `Option<u32>` and its own doc
says `None` is *"nothing to draw, the device refuses the wait, or the
mapping fails — every one of which is 'the GPU has no answer', never a
wrong answer"*. The call site spends that distinction:

```
    .unwrap_or(IdMap::NOTHING)
```

`IdMap::NOTHING` is the id that means **the cursor is over empty
space**, which is an answer the pass did not produce. It is stored
into the answer channel beside the query's serial, so the id log reads
it as a fresh, valid reply.

## It is not silent, which is worse than it sounds

`crate::idpass::disagreement` reads that word back and compares the
two picking paths by name. `id == IdMap::NOTHING` gives `from_gpu =
IdAnswer::Nothing` (`idpass::IdAnswer::of`); a ray that DID find a
face then does not contain it (`disagreement`'s `agrees` is
`from_ray.is_empty()` for that arm), and the frame pushes
`Disagreement { from_gpu: IdAnswer::Nothing, from_ray: vec![name] }`
onto the status line as *"id buffer nothing"*. `IdAnswer::Nothing`'s
doc now cites this row: the arm holds a failed readback as well as
empty space until the channel word tells them apart. So a device that could not answer is reported to the
reader as **the id pass and the ray disagreeing about the picture** —
a sentence about the model, blaming the half of the crate that was
working. The GUI0 role inversion recorded at `disagreement` makes the
ray authoritative precisely so the id pass can contradict it out loud;
contradicting it because a readback failed is the one thing that
argument does not cover.

## What the fix is

`read_id_at`'s three `None`s are not one fact — "nothing to draw" is a
genuine empty answer and the other two are a device failure — so the
repair is at `read_id_at` as much as at its call site: a door that
distinguishes *no geometry* from *no answer*, and a call site that
stores an answer only for the first. There is already a channel for
the second: `frame::index_badge`'s neighbour set, or a notice of its
own.

## Fence

`crates/viewer/src/gpu.rs` and `crates/viewer/src/idpass.rs` — VIEW's.
No test can execute either today: the readback needs a real adapter
(`gpu::tests::every_pass_builds_on_a_real_device` is the crate's one
row that has one, and it is the expected `--lib` red on a box with no
Vulkan). Whatever shape the fix takes, the part that is testable
headlessly is the door's vocabulary, not the readback.
