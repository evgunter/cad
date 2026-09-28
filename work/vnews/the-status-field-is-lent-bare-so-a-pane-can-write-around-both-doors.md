---
id: the-status-field-is-lent-bare-so-a-pane-can-write-around-both-doors
kind: issue
title: ViewerBehavior lends the panes the status line as a bare Option<Message>, so a pane can still write around both frame doors
status: open
opened: 2026-09-28
priority: P1
cost: M
refs: [ranked-and-unranked-verdicts-are-one-type]
design: true
---

Found by `ranked-and-unranked-verdicts-are-one-type`'s sweep, as the
blind spot of its own fix.

## What is left after the two verdicts became two types

`frame::deliver` takes only a policy's `StatusUpdate` and
`frame::apply` only the ranking's `LineVerdict`, so a policy's verdict
can no longer reach the field through the wrong door: that is a type
error now (the `compile_fail` doctest on `frame::deliver`). What the
types do NOT close is the field itself.

`app::ViewerBehavior` lends every pane `status: &'a mut
Option<frame::Message>` (`crates/viewer/src/app.rs`, the
`ViewerBehavior::status` field), because the retiring half of
`deliver` needs it. With a bare `Option<Message>` in hand a pane can
still

- assign it: `*status = Some(message)`, or
- build its own `frame::LineVerdict::Show(message)` and `apply` it —
  `LineVerdict`'s variants are public, so "only `frame_status`
  answers in it" is prose on the type,

and either puts a sentence on the line the ranking never saw.
`crates/viewer/src/frame.rs`'s module header (*"Nothing enforces the
'but one'"*) says so. Swept 2026-09-28 (every assignment to a `status`
field or `*status` under `crates/viewer/src`): no production site does
either today. The writers are `app::ViewerApp::apply_status` (through
`apply`), `pane::viewport::land` and the id pass (through `deliver`),
and the startup initializer
(`work/vseam/startup-notices-need-holding-to-badge.md`). So this is a
missing guarantee, not a live defect.

## The shape of a fix, and why it is a choice

A `frame::StatusLine` newtype over a private `Option<Message>`, whose
only mutators are `deliver` and `apply` (plus a read for the toolbar),
would make both spellings above fail to build, and would let
`LineVerdict` be made only inside `frame`. It touches `ViewerApp`'s
field, `ViewerBehavior`, the toolbar read, the startup initializer
(the one writer that assigns) and every test that seeds a line. The
startup exception therefore has to be answered first or alongside, and
the weighing is whether a sealed line is worth turning that exception
from a comment into a compile error.
