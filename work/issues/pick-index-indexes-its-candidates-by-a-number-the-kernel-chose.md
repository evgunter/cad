---
id: pick-index-indexes-its-candidates-by-a-number-the-kernel-chose
kind: issue
title: pick_for indexes its candidate list with a position editor-core chose, and a bad one panics
status: open
opened: 2026-09-21
priority: P3
cost: E
refs: [3007]
---

Found by the whole-file read (`docs/prompts/reviewer-style-lane.md`
Q8) during #3007's review. Pre-existing; #3007 does not touch it.

## Finding

`crates/viewer/src/pickindex.rs`, `PickIndex::pick_for`:

```
        let at = |face: &FaceAnswer<(RecipeNodeId, u32, StableName)>| PickHit {
            t: face.span.t,
            ...
            ..candidates[face.member].clone()
        };
```

`face.member` is a position `editor_core::resolve::answer_of` chose
into the `pairs` vector this function built. A member past the end
panics the UI thread. Nothing here checks it, and nothing in the
signature says the answer's positions index the input — it is a
convention between two crates, held by neither's type.

The file's own module header is what makes this worth a row rather
than a shrug: its whole argument is that a pick must not be able to
pair a mesh with the wrong node, and that `NodePick` *"offers no door
through which it could"* (#1098). This is the same class one level
out — a position from elsewhere used as an index — and it is the one
place in the file where a bad one is not refused but fatal.

## Why here rather than on a program's slate

`pickindex.rs` is claimed by five programs and none of their charter
tests covers this. VGEOM's is *a wrong number, or no number, reaches
the screen* — a panic is neither; VSEAM's is state that outlives its
frame; VDOC's changes no behaviour. `work/README.md`'s genuine case
for `work/issues/`: the owner is undecided rather than un-notified.

## What a fix has to decide

What the door answers when the kernel names a member it did not hand
over. `Ok(None)` reads as a miss the ray did not make;
`HitTestError` has no arm for *the answer indexed nothing*, and
minting one reaches `crates/editor-core` (EDIT's and MSOLVE's).
