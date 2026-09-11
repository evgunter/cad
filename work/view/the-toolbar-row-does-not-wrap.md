---
id: the-toolbar-row-does-not-wrap
kind: issue
title: The toolbar is one non-wrapping horizontal row of twelve controls, so a narrow window pushes the last of them out of reach
status: open
opened: 2026-09-11
refs: [gesture-drags-have-no-cancel-door]
---


Found by the cancel-door unit (2026-09-11), which added the eleventh
and twelfth controls to the row and then wrote a clause claiming its
doors are drawn in every state. The clause is now scoped to what was
actually checked — the selection, the standing and the evaluation — and
this is the part it cannot claim.

## What happens

`crates/viewer/src/app.rs:1148` opens the toolbar as `ui.horizontal`,
not `ui.horizontal_wrapped`. egui lays a non-wrapping row out on one
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
