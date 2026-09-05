---
id: projection-fault-has-no-sweeper
kind: issue
title: The projection fault this unit holds can go permanently stale, and neither of the two new app fields has a row
status: open
refs: [1957, news-and-standing-facts-are-orthogonal-axes]
opened: 2026-09-05
---



Found by the style review of #1957, which introduced both fields.

## 1. `projection_fault` can go permanently stale — a behaviour regression

`crates/viewer/src/app.rs` holds `projection_fault: Option<CameraError>`
and `frame::projection_badge` reads it. **It is written in exactly one
place**, `crates/viewer/src/pane/viewport.rs`'s `viewport_ui`, at the
`view_projection` match — `None` on success, `Some` on refusal.

`viewport_ui` returns at `crates/viewer/src/pane/viewport.rs:56-58`
when `ViewportSize::aspect()` is `None` (either extent zero), **before
either writer**, and it is not called at all when the viewport pane is
not drawn. So a pane dragged to zero extent, or tabbed away, leaves
whatever the field last held standing forever, and the toolbar keeps
badging a projection refusal for a camera nobody is projecting.

**The old code had a sweeper and the new code does not.** The same
condition used to leave a stale *sentence* on the status line, which
the next `StatusUpdate::Clear` took. A badge is read from held state
and no `StatusUpdate` reaches it, so nothing sweeps this.
`app.rs`'s field doc argues that the one-frame lag between the pane
writing and the toolbar reading is benign; it does not consider the
NO-frame case, and that is the gap.

**Two shapes of fix, and neither is obviously right yet.** Clearing the
fault at the zero-aspect return closes one arm and not the other. The
honest shape is that the field means "what the viewport said the last
time it drew", so the application clears it on a frame the viewport did
NOT draw — the `profile_form_drawn` latch is the pattern, and it is a
third piece of app-gated state.

## 2. Neither new field has a row

`scene_fault` and `projection_fault` are two small state machines in
`app`-gated code, and no test names either. That is the exact condition
`crates/viewer/src/frame.rs`'s header exists to condemn: *"all three
lived in `app`-gated code no test can execute — so the crate's own
claim that everything between event conversion and painting is
exercised by `tests/` was false about exactly the rules most likely to
be wrong."*

The badge VALUES are rowed (`crates/viewer/tests/frame_policy.rs`) —
their subjects, their labels and their silence. What is unrowed is the
set/clear discipline on the two fields, which is where §1 above
actually lives: a row that could see §1 is a row over "when is the
fault written and when is it cleared", and there is nowhere headless to
put one until that discipline is a value rather than two assignments in
a draw path.
