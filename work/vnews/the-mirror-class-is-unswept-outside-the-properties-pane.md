---
id: the-mirror-class-is-unswept-outside-the-properties-pane
kind: issue
title: "a control drawn usable over a refusal its op will give: swept in properties.rs only, four files unswept"
status: open
opened: 2026-09-20
refs: [a-disabled-control-says-why-in-four-shapes, the-hide-toggle-is-drawn-over-a-refusal-the-op-will-give, the-unit-picker-is-offered-on-a-slot-whose-notation-is-not-the-users]
priority: P3
cost: D
---

`a-disabled-control-says-why-in-four-shapes` parked **"a control drawn
as usable that refuses on click — the rule's mirror image"** as a shape
no pass in it could see, with one named instance that it called
unreachable in practice. `vnews/properties-controls-read-their-refusals`
(PR #2961) swept the class in **one file** and found two members there
— the hide toggle (fixed) and the slot unit picker (filed) — so the
class is real, populated, and measured over a fifth of its ground.

This row is the rest of the sweep. It is a census, not a fix.

## The instrument, stated so it can be re-run

For each control-producing call — `ui.button`, `small_button`,
`checkbox`, `link`, `selectable_label`, `add_enabled`,
`ui.add(number_field…)`, `radio_value`, `ComboBox` — and each
`ops.push(SessionOp::…)` beside it, name the op, find the door
`DocSession::perform` routes it to, and compare **the door's admission
test** with the control's enabling condition. Follow the `widgets.rs`
helpers that push on a panel's behalf (`delete_button`, `drag_ops`,
`drag_gesture_ops`, `vec3_row_ops`) into their own bodies.

**Unswept ground**, with what each is expected to hold:

- `crates/viewer/src/pane/create.rs` — VSEAM's. The largest population
  of controls in the crate and the file the census's only named
  instance is in.
- `crates/viewer/src/pane/profile.rs` — eight `add_enabled` sites, all
  reading `ShapeEdits::free()`; the question is whether the editor's
  own doors refuse anything `free()` admits.
- `crates/viewer/src/widgets.rs` — eight more, plus every helper that
  pushes an op for someone else.
- `crates/viewer/src/app.rs` — VSEAM's. The toolbar: Undo/Redo, the
  choosers, the cancel doors.
- `crates/viewer/src/pane/features.rs`, `tree.rs` — the tree's row
  controls, which push selection and delete.

A finding in a VSEAM file is filed there, not fixed here.

## What the one swept file predicts

Both members found there had the same shape: **a control whose gate is
a NARROWER test than its door's**. The hide toggle gated on the kind
test while the op ran the full admission test; the unit picker gated on
nothing while the op refuses a non-literal. So the cheapest form of
this sweep is: for every control, is the gate the door's own test, or a
test that merely implies it is worth drawing the control at all?

## Two blind spots this instrument has, named in advance

- **Refusals a door discovers late.** The comparison is against the
  door's ADMISSION test. `probe::probe_bounds` can still refuse after
  its gate passes, and `commit`'s `EditError`s are document-consistency
  answers no chrome could pre-compute. A control whose door refuses for
  something the chrome COULD have computed but the door only reaches at
  the end will not match.
- **A fifth op-pushing helper.** The four in `widgets.rs` were followed
  by hand. A helper added later that pushes an op under another name is
  invisible to the pattern, and nothing re-derives the list of four.

The third blind spot PR #2961 named — **a gate that agrees with its
door by coincidence rather than by reading the same value** — turned
out to have a member inside that lane's own diff and is filed with it:
`work/vseam/a-panels-gate-reads-the-previewed-document-while-its-door-reads-the-committed-one`.
