---
id: free-move-in-flight-refusal-has-no-reachable-producer
kind: issue
title: "DisplayFault::FreeMoveInFlight is reachable: the keyboard is the second hand, and #2358 added a second producer"
status: closed
opened: 2026-09-11
closed: 2026-09-11
refs: [gesture-drags-have-no-cancel-door, escape-commits-a-free-move-instead-of-abandoning-it, a-keyboard-bump-lands-and-closes-the-pointers-own-probe]
---


## Answered: it is reachable, and the answer is a row

`crates/viewer/src/widgets.rs`'s
`a_keyboard_bump_begins_a_second_probe_under_a_held_drag` drives the
probe field's three `DragValue`s through the real [`drag_ops`] against a
headless `egui::Context` and reads the operations back. A pointer press
and a move open a probe (`["begin", "preview"]`); every frame after it
carries keyboard events only, and a Tab/ArrowUp pair on a component the
pointer is not holding emits `["begin", "preview", "commit"]` with no
commit and no cancel between it and the first begin. That second
`BeginFreeMove` is performed under an open probe, which is exactly what
`DisplayState::begin_free_move` refuses
(`crates/viewer/src/display.rs:746-748`), and the refusal renders as
*"finish the free-move first"* (`display.rs:224`).

The structural fact the row rests on is egui's, not this crate's:

- **One pointer is one drag, and that is not a limit a search could
  have missed.** `egui-0.36.1/src/interaction.rs:24-40` carries
  `dragged`, `drag_started` and `drag_stopped` as a single
  `Option<Id>` each, and `interact` maintains exactly one of each per
  frame. So the item's *second pointer (egui multi-touch)* candidate
  is dead structurally: multi-touch feeds `MultiTouchInfo`, which is a
  zoom/rotate aggregate and never a second widget drag.
- **The keyboard is the second hand, and it is not a pointer at all.**
  `egui-0.36.1/src/widgets/drag_value.rs:462-466` puts a `DragValue`
  into edit mode the frame it takes focus — deliberately, "for screen
  readers" — and answers ArrowUp/ArrowDown there. Nothing in egui's
  focus or key handling consults the pointer, and `interact` aborts a
  drag on Escape alone (`interaction.rs:137-141`). So Tab and ArrowUp
  reach a component while the pointer holds another, and
  `drag_ops`' typed arm (`crates/viewer/src/widgets.rs:69-73`) spells
  that as a whole begin/preview/commit.

## And the item's premise was already incomplete

`DisplayFault::FreeMoveInFlight` has a **second producer** that needs no
second `BeginFreeMove` at all: `crates/viewer/src/session.rs:1089-1090`
raises `Refusal::Display(DisplayFault::FreeMoveInFlight)` for every
operation `SessionOp::permitted_during_free_move`
(`crates/viewer/src/session/op.rs:852`) answers `false` for, which is
`Open` and `NewDocument` (`op.rs:854`). That arrived with #2358, after
the trace this item was written from.

It is reachable by the same second hand, and by a wider one: egui gives
any click-sensing widget a click with no pointer, from keyboard focus
plus Space/Enter or from an AccessKit `Action::Click` request
(`egui-0.36.1/src/context.rs:1464-1478`, read by `Response::clicked`
at `response.rs:183-184`). So the toolbar's New…/Create and Open…
controls are activable while a free-move drag is held.

## What this does NOT land

`gesture-drags-have-no-cancel-door`'s honesty argument does not reach
this arm. On every route above the probe has a pointer behind it — the
user is holding the drag — so *"finish the free-move first"* is an
instruction they can follow by releasing the button. And the door
exists anyway: `DocSession::cancel_doors` draws *"Cancel free-move"*
enabled exactly while `DisplayState::probing` is `Some`
(`crates/viewer/src/session.rs:660`).

The stranded case the item asked about — a probe open with no pointer
behind it — was NOT found, and the search that did not find it is
worth recording rather than repeating: `prune` runs on every document
transition (`session.rs:1611` and `session.rs:1994`) and kills a
gesture whose instance stopped passing `free_move_check`, which is the
same predicate that replaces the field with `ui.weak(fault)` in
`instance_ui`; and `Open`/`NewDocument`, the two doors that would drop
the display state whole, are refused under a probe. The hole left is
the SELECTION, which no prune covers: `instance_ui` is drawn only for
`selection().node()`, so a `Select` performed under an open probe would
take the field away with the drag still live. Every `SessionOp::Select`
producer in `crates/viewer/src/` today is a pointer click
(`pickindex.rs:1551`, `pane/features.rs:60` and `:109`,
`pane/properties.rs:127`, `:200`, `:685`, `app.rs:1112`) and a click
cannot land while the same pointer holds a drag — but a keyboard- or
AccessKit-activated one could, by the mechanism above. Nothing turns on
it for this row, which is closed either way.

## Closed

Reachable. No deletion question arises: the guard fires.

Residue, each its own file: `escape-commits-a-free-move-instead-of-
abandoning-it` and `a-keyboard-bump-lands-and-closes-the-pointers-own-
probe`.
