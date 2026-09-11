---
id: escape-commits-a-free-move-instead-of-abandoning-it
kind: issue
title: Escape during a free-move drag commits the probe instead of abandoning it
status: open
opened: 2026-09-11
---



Found by the free-move reachability unit (2026-09-11), driving the
probe field against a headless `egui::Context`.

egui aborts a drag on Escape and on nothing else:
`egui-0.36.1/src/interaction.rs:137-141` clears `dragged` there, which
makes `drag_changed` true, which makes `drag_stopped` name the widget.
So the key that every other control in this chrome spells *abandon*
reaches `drag_gesture_ops`' release arm
(`crates/viewer/src/widgets.rs:95-98`) as an ordinary release, and the
chrome emits `CommitFreeMove`.

Measured, with the harness now in `crates/viewer/src/widgets.rs`'s test
module: press and move on the probe's x component gives
`["begin", "preview"]`; the next frame carrying only
`egui::Key::Escape` gives `["commit"]`. The probed frame is landed into
`DisplayState::moves`, not discarded.

That is the opposite of what the chrome offers beside it: `cancel_doors`
draws *"Cancel free-move"* precisely so a probe can be abandoned
(`crates/viewer/src/session.rs:660`), and `SessionOp::CancelFreeMove`
exists for it.

The same reading applies to the value gesture, whose triple runs through
the same function — not checked here, and worth checking with the fix.

## The fork, because it is not obviously a bug in this crate

Escape is egui's abort, so one repair is to read it where the triple is
mapped: `drag_gesture_ops` could emit the cancel operation rather than
the commit on a `drag_stopped` frame that carries an Escape press. That
needs a fourth operation parameter and an `Option` for the vocabularies
that have no cancel. The other is to decide that a probe ends by
landing whatever it previewed, whichever key ended it, and to say so —
in which case the row closes as ratified rather than fixed.
