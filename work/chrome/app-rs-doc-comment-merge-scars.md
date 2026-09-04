---
id: app-rs-doc-comment-merge-scars
kind: issue
title: Three doc-comment merge scars in viewer/src/app.rs leave apply_status undocumented
status: parked
opened: 2026-09-04
refs: [1776]
blocked_on: [viewer-session-god-module-split]
---


## Finding

Three doc comments in `crates/viewer/src/app.rs` are merge scars — a
doc block spliced onto another one, or onto itself — and one of them
leaves a function undocumented. Line numbers are as of PR 1776's head;
the function names are the durable anchors.

1. **`tip_mark`, `app.rs:247` — a doc line spliced onto itself.** The
   line reads
   `/// **How big the tip marks in a profile preview are**/// **How big the tip marks in a profile preview are**, in sketch-plane`
   — one summary, twice, with the second copy's `///` in the middle of
   the first. Renders as one run-on sentence in rustdoc.

2. **`perform_batch`, `app.rs:1722-1723` — two summaries, the first
   describing a function that is not there.** The block opens
   `/// Perform one operation and record what it refused.` and then
   `/// Perform one frame's whole batch of operations, keeping the
   refusal worth showing.` The first line is the pre-batch signature's
   summary, kept above its own replacement. Everything below it is
   about the batch, and the paragraph's whole point is that this is
   NOT one assignment per op — so the stranded line says the opposite
   of what the doc argues.

3. **`remember_theme` / `apply_status`, `app.rs:1809-1811` and
   `:1840` — one function's doc glued on top of another's, leaving the
   second undocumented.** The block above `remember_theme`
   (`app.rs:1827`) opens with three lines that describe
   `apply_status`: *"Apply a policy verdict to the status line — the
   one place a `StatusUpdate` becomes the field, shared by the batch
   policy and the dialog policy so neither hand-assigns."* The rest of
   the block is genuinely `remember_theme`'s. `apply_status` itself,
   at `app.rs:1840`, then has no doc comment at all — in a crate whose
   every other private method carries one.

## Why it is filed rather than fixed

Not PR 1776's — all three predate it, and it touched none of these
functions. The style review that found them was the first read of
`app.rs` end to end; nothing in the per-unit process would have
surfaced them, because each sits in a region no unit had reason to
open. One file for the three because they are one class of defect with
one fix.

Whoever takes it should also look for a fourth: the class is "a doc
block survived a merge that replaced the item under it", and three
found in one read is not evidence there are only three.

## Home

CHROME (`crates/viewer/src/*`).
