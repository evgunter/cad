---
id: a-swallowed-ray-refusal-is-announced-as-a-picking-disagreement
kind: issue
title: A hit-test refusal the viewport drops is announced as the two picking paths disagreeing
status: open
opened: 2026-09-22
priority: P2
cost: D
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
