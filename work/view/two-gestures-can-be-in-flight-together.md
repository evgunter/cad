---
id: two-gestures-can-be-in-flight-together
kind: issue
title: session::Gesture and display::FreeMoveGesture share a field name and no guard, so a slider drag and a free-move probe can overlap
status: open
opened: 2026-09-04
refs: [viewer-session-god-module-split, save-is-not-gesture-guarded]
---


Found by the whole-file read that opened
`viewer-session-god-module-split` (2026-09-04).

## What happens

Two unrelated types are both reached as `self.gesture`:

- `session::Gesture` (`crates/viewer/src/session.rs:1558`) — a slot or
  parameter drag;
- the free-move gesture on `DisplayState`
  (`crates/viewer/src/display.rs`, read at 555, 574, 599, 619, 658,
  672).

Same field name, different owner, different type, no relation. That is
two spellings of one concept across two files, and it is the reason
the second half is easy to miss: the four `*FreeMove` operations are
**not** guarded against the session gesture (they are among the
unguarded set in `save-is-not-gesture-guarded`), so a slider drag and
a free-move probe can be in flight at the same time.

Whether that overlap is reachable through the real UI depends on
whether the panels can accept a drag while the viewport holds a probe;
whether it is *intended* is not stated anywhere. What is certain is
that nothing enforces either answer.

## Why it matters for unit 1

The charter asks whether gesture-safety becomes data. If it does, this
is the case that decides the shape of the datum: one flag is not
enough if there are two gestures, and a `gesture_safe` predicate that
silently means "safe against the slot gesture only" would be a table
that reads as complete and is not — the exact failure the table exists
to prevent.

## Home

VIEW's: `crates/viewer/src/session.rs` and `display.rs`.
