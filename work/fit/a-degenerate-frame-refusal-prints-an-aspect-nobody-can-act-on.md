---
id: a-degenerate-frame-refusal-prints-an-aspect-nobody-can-act-on
kind: issue
title: A degenerate-scene frame refusal prints the viewport aspect to seventeen digits and calls it the half a reader can act on
status: open
opened: 2026-09-24
refs: [zoom-to-fit-frames-no-committed-profile]
---


Reported by Ev from the running viewer on 2026-09-24, as the sentence
the toolbar showed when Zoom to fit met an empty scene:

> camera: the scene bounds give a radius of 0, which is not a positive
> extent to frame against (from frame the given bounds at aspect
> 1.4754263191763193)

The extent half — that a scene holding only a profile, committed or
being authored, frames nothing — is
`zoom-to-fit-frames-no-committed-profile`, on this slate. This row is
only about **the sentence**, which is wrong on its own terms and stays
wrong for any route that still reaches this refusal after that row
lands.

## What the sentence does

`crates/viewer/src/camera.rs`, `impl Display for` the camera fault, the
`Self::Frame { aspect, .. }` arm, writes *"frame the given bounds at
aspect {aspect}"* — the op's own description, and `aspect` through
plain `Display`, so a reader gets seventeen significant digits of the
viewport's width-to-height ratio. `DegenerateScene { radius }` supplies
the first clause.

## Why it is wrong, argued against the code's own argument

The comment above that arm defends printing the aspect: the errors that
provoke it *"say what was wrong with the box themselves … while `aspect`
is rendered because the viewport shape is the half a reader can act
on."* **For `DegenerateScene` that is false.** Nothing about the
viewport's shape makes a radius-zero scene frameable; the aspect is
irrelevant to the refusal and there is nothing for a reader to act on in
it. The argument is sound for `CameraError::Unfittable`, where the
stand-off really does depend on the aspect, and it was written as though
it held for every error that reaches the arm.

**And the same file already states the rule the aspect breaks.** The
`scene_radius` arm prints in scientific notation rather than plain
`Display` because the value at full precision is *"a sentence nobody
can read, about a number nobody can read."* The aspect gets no such
treatment, and `1.4754263191763193` is exactly that sentence.

A third, smaller one: *"(from frame the given bounds …)"* names the
operation as the camera module spells it, not as a reader invoked it —
they clicked **Zoom to fit**.

## What a fix has to decide

Whether the `Frame` arm renders the aspect at all when the error is not
aspect-dependent — which means the arm reading which error it wraps,
rather than one sentence serving both — and, where the aspect does
belong, at what precision. Out of scope here: whether this refusal
should reach a reader at all for an empty scene, which is the sibling
row's question.

## Home

`crates/viewer/src/camera.rs` is this program's. The sentence a reader
sees is VNEWS's subject by charter, so a VNEWS lane may reasonably take
it — announce the crossing either way.
