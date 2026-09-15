---
id: the-toolbar-row-does-not-wrap
kind: issue
title: The toolbar is one non-wrapping horizontal row of twelve controls, so a narrow window pushes the last of them out of reach
status: closed
opened: 2026-09-11
refs: [gesture-drags-have-no-cancel-door]
closed: 2026-09-14
---


Found by the cancel-door unit (2026-09-11), which added the eleventh
and twelfth controls to the row and then wrote a clause claiming its
doors are drawn in every state. The clause is now scoped to what was
actually checked — the selection, the standing and the evaluation — and
this is the part it cannot claim.

## What happens

`ViewerApp::ui`'s top panel opened the toolbar as `ui.horizontal`, not
`ui.horizontal_wrapped` (`crates/viewer/src/app.rs:1157` when this was
filed; the body cited `:1147`, which was the `impl eframe::App` header
two lines of context above). egui lays a non-wrapping row out on one
line and clips what does not fit, so on a narrow window the controls at
the right-hand end are not merely small — they are unreachable, with
nothing saying so.

The row holds twelve controls: the document name, `New…` (or the name
field with `Create` and `Cancel`), `Open…`, `Save As…`, `Undo`, `Redo`,
**`Cancel drag`**, **`Cancel free-move`**, `Zoom to fit`, and the
progress indicator's own control (`Cancel` while evaluating,
`Re-evaluate` after a cancel). The two named in bold are this unit's,
and they sit in the clipped half.

**Why it matters more for these two than for the rest.** Every other
control has a second route or a recoverable failure: a dialog can be
reopened, `Zoom to fit` re-run, an evaluation re-requested. A cancel
door is the ONE exit from a state in which every other operation
refuses `Refusal::GestureInFlight`, which is exactly why it was put in
the panel that is always drawn. A door that is drawn and off the right
edge is the defect the siting argument was supposed to answer.

## What is NOT established

**That any real window is narrow enough.** Nothing here was measured:
this crate has no headless egui harness, so neither the width the row
needs nor the width a browser or a tiled window actually gives it was
taken. The clipping behaviour is egui's documented layout rule, not an
observation of this toolbar.

## The candidate answers, and what each costs

- **`ui.horizontal_wrapped`** — one word, and the row becomes two or
  three lines when it must. It changes the toolbar's look at every
  width for a hazard nobody has measured.
- **A `ScrollArea::horizontal`** — keeps one line and makes the
  overflow reachable; adds a scrollbar a reader has to notice.
- **Measure first.** Whatever the repair, the thing missing is an
  instrument: the width this row asks for is a number egui can be asked
  for, and no row in this crate asks it.

## Home

VIEW's: `crates/viewer/src/app.rs`.

## Measured, and answered (2026-09-14)

**Both of this row's unestablished facts are now numbers**, taken by
laying the REAL toolbar out in a headless `egui::Context` —
`ViewerApp::assemble` is the half of startup that needs no graphics
device, split out for this, and `ViewerApp::toolbar_ui` is the row
itself, extracted from the 280 lines that were inline in
`ViewerApp::ui`. "This crate has no headless egui harness" had already
stopped being true when this was written: `widgets.rs`, `pane/view.rs`
and `pane/viewport.rs` all drive one.

- **The row's natural width is 964 points** (default style, default
  document, no gesture in flight, no status line — so a LOWER bound on
  what a user sees: a status notice or a longer document name only adds
  to it).
- **A 400-point window** — an upright phone browser, which `run_web`
  ships this same toolbar into — gives the panel 384. **580 of the
  row's 964 points, 60% of it, laid out past the right edge.**
- It is not only a phone: 964 points does not fit a desktop window
  tiled to half of a 1920-point screen either (960), and only just fits
  a 1024-point one.

The list of controls above is also short. The row holds, beyond the
twelve named, a theme picker (`ComboBox` `viewer_theme`, there since
`cf2164600f`, 2026-09-03 — before this was filed), up to three badges,
and the status label — all of them to the RIGHT of the two cancel
doors, which is why the doors sit near the middle of a 964-point row
rather than at its end.

**The repair is `ui.horizontal_wrapped`.** The cost this row feared —
"it changes the toolbar's look at every width" — is not real: egui's
wrapped horizontal layout wraps only when the content does not fit, so
at every width where the old row fitted the new one is identical. The
`ScrollArea::horizontal` alternative was refused on the doors' own
argument: a scrolled-off control is still not visible, and a cancel
door that a user must first discover a scroll affordance to reach is
the same defect with an extra step.

Held by two rows in `ViewerApp::toolbar_ui`'s neighbour module:
`the_toolbar_asks_for_more_width_than_a_narrow_window_gives` (the
wrapping answers something — if the toolbar ever shrinks to fit, this
reads red and both rows should be retired) and
`the_toolbar_wraps_rather_than_running_past_a_narrow_window` (it stays
inside the window it is given). Neither pins a pixel width of the row;
the only number either holds is the window's.
