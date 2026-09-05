---
id: save-permitted-row-argues-only-half-of-save
kind: issue
title: The Save row's argument for being permitted mid-gesture covers save_path and not the resolver rebind
status: open
opened: 2026-09-04
refs: [save-is-not-gesture-guarded]
---


Residue of `save-is-not-gesture-guarded`, filed at the moment that item
closed rather than disclosed inside its prose — `work/README.md` is
explicit that a residue named only in a closing item's body reads as a
record of work done, is invisible to the re-homing sweep, and dies with
this directory.

## What is wrong

`SessionOp::permitted_during_value_gesture`
(`crates/viewer/src/session/op.rs:650`) records `Save` as permitted
mid-gesture, and argues it at `:642-645`:

> [`SessionOp::Save`], which writes the COMMITTED history and so
> ignores a preview that is not in it.

`DocSession::save` (`crates/viewer/src/session.rs`) has **two**
effects, and that sentence is about the first:

1. `docio::save_path(path, &self.history, self.tol)` — the committed
   history, which is what the argument covers and what
   `crates/viewer/tests/review_gui3_r2.rs`'s
   `a_save_taken_mid_gesture_writes_the_committed_document_not_the_preview`
   pins;
2. on a **save-as whose parent directory differs**, a rebind of
   `self.resolver` and a `request_eval()`. `request_eval` submits
   `self.doc()`, which mid-gesture is the **scratch** — so this half
   acts on the preview, which is precisely the thing the argument says
   `Save` ignores.

The behaviour is sound (the directory rule following the file, applied
to the document actually on screen — adjudicated on the closing item),
so **this is a stale argument, not a defect**. It matters because the
table exists to be read: a reader checking whether `Save`'s `true` is
justified gets a reason that does not reach the whole function, and
this program has now been bitten eight times by prose that outran its
tree.

## What a fix looks like

One or two sentences at `session/op.rs:642-645` naming both effects and
why the second is safe. Nothing else — no code.

**Rides the next lane that touches `session/op.rs`** rather than
getting a branch of its own; it is a comment.

## Home

VIEW's: `crates/viewer/src/session/op.rs`.
