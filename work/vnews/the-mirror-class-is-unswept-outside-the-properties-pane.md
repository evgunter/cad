---
id: the-mirror-class-is-unswept-outside-the-properties-pane
kind: issue
title: a control drawn usable over a refusal its op will give: swept in properties.rs only, four files unswept
status: closed
opened: 2026-09-20
refs: [a-disabled-control-says-why-in-four-shapes, the-hide-toggle-is-drawn-over-a-refusal-the-op-will-give, the-unit-picker-is-offered-on-a-slot-whose-notation-is-not-the-users]
priority: P3
cost: M
closed: 2026-09-28
branch: vnews/the-mirror-class-census
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

## Closed

The census is taken at merge base `f4e9aa68b`, by reading only; no code
changed. Line numbers are approximate and ride beside the names.

**Instrument, as run.** This is the instrument above, with three
changes that the "What the instrument got wrong" section argues:

1. The op-pushing helpers are found by the pushes themselves, not by a
   name list. A helper that returns a `SessionOp` built elsewhere
   pushes it as `self.ops.push(op)`, so grep for every `ops.push(` and
   every fn returning `SessionOp`, `Vec<SessionOp>` or
   `Result<SessionOp, _>`.
2. Each control is also compared with its door's LATE refusals
   (`commit`'s `EditError`, a value read at the door), wherever the
   chrome holds what would compute them. That is the second pass on
   blind spot 1.
3. For each gate, record which document it reads (landed, committed or
   previewed) against the one the door reads.

The session-wide `GestureInFlight`/`FreeMoveInFlight` pre-gate is left
out throughout: `a-gated-control-does-not-read-the-held-gesture-refusal`
holds it for every control.

**Members: 17 controls, filed as 7 rows.**

| file | controls | row |
|---|---|---|
| `pane/create.rs` | Extrude; Commit revolve, boolean, split, transform, pattern, projection, duplicate; Commit blend (9) | `work/vseam/the-creation-tools-commit-buttons-are-live-over-a-pick-their-door-refuses-by-kind` (P3, M, design) |
| `pane/create.rs` | Commit duplicate, again, over its value refusals | `work/vseam/the-duplicate-button-is-live-over-its-doors-value-refusals` (P3, M) |
| `pane/create.rs` | Add datum (axis in sketch; frame on face); Add profile on an existing frame (3) | `work/vseam/a-held-pick-is-admitted-off-the-landed-run-while-its-door-reads-the-committed-document` (P3, E) |
| `pane/profile.rs`, `pane/create.rs` | Apply (edit door); Add profile, when its preview is absent (2) | `work/vseam/an-absent-profile-preview-reads-as-admitted` (P3, M) |
| `pane/create.rs` | the part chooser's entry buttons (1 control) | `work/vseam/the-part-chooser-keeps-its-snapshot-across-a-document-swap` (P3, E) |
| `pane/features.rs` | the tree's "shown" checkbox (1) | `work/vnews/the-feature-trees-shown-checkbox-is-drawn-over-a-refusal-the-op-will-give` (P3, E) |
| `pane/properties.rs` (reached through `widgets.rs`'s `delete_button`) | Delete on a face or edge selection (1) | `work/vnews/a-face-or-edge-delete-is-live-on-a-feature-the-committed-document-no-longer-holds` (P3, E) |

By file: `pane/create.rs` 14 (Commit duplicate and Add profile each
counted once, though each is in two rows), `pane/profile.rs` 1,
`pane/features.rs` 1, `pane/properties.rs` 1, and 0 in `widgets.rs`
itself, `app.rs` and `tree.rs`.

The rows follow three shapes. Two carry the prediction above (the gate
is a narrower test than the door's): the kind gate, and the duplicate's
value gate. **The third is new: a gate that reads the landed run where
its door reads the committed document** (held frame and face picks,
face delete, the tree's checkbox). The absent-preview row is a fourth
of its own, a gate that treats "not computed" as "admitted". The part
chooser is the third shape with a scan in place of a landing.

**Non-members checked.**

- `app.rs` (`toolbar_ui`, `checks_window`):
  - Create (new document): the gate is `Refusal::new_document_name`,
    the door's own test.
  - Undo and Redo: `Refusal::nothing_to_step` is the `step` door's own
    predicate (`History::can_undo`/`undo` read one field).
  - Cancel drag and Cancel free-move: `cancel_doors` is the door's own
    in-flight test.
  - Cancel (evaluation) and Re-evaluate: their doors never refuse.
  - The checks window's select-root buttons: the `Select` arm never
    refuses.
  - Open…: its refusal is late IO.
  - Save As…: late IO. Its `DocIoError::Persist` arm is pure
    (`persist::save` validates and replays the history's path edits),
    and it looks reachable only on an invariant break. Not chased
    further.
  - No keyboard shortcut in `app.rs` pushes an op.
- `pane/create.rs`:
  - Commit mate: `AddMate` goes to `commit` with no admission test.
    `MateTool::proposal`'s faults are notices, and no op is pushed.
  - Add datum (plane, frame, axis, point): `add_datum` admits every one.
  - Add profile on the new XY frame: its preview runs the commit's
    ladder.
  - Everything else is tool openers, cancel and clear buttons, and
    radios, checkboxes, number fields and unit pickers that write
    drafts only.
- `pane/profile.rs`:
  - Revert: its local door cannot refuse, because `sync_profile_edit`
    drops a draft whose reload would.
  - Apply's `ProfileEditStale`, `ProfileRestructure` and `guard_driven`
    arms cannot be reached. The draft reloads on a changed program,
    structure is locked, and a driven argument never loads.
  - The step controls (remove, up, down, +, verb ComboBox, Add step,
    Clear) and the notation pickers write the draft only.
- `widgets.rs`:
  - `drag_ops` and `drag_gesture_ops` inherit their callers' gates.
  - `vec3_row_ops`: its caller in `instance_ui` is gated on
    `free_move_check(committed_doc())`, which `begin_free_move` runs.
  - `value_field_ops` on a parameter row: rows exist only for declared
    parameters, and the value's kind comes from the declaration.
  - `value_field_ops` on a driven slot: the number and drag doors
    refuse `DrivenByExpression`. That is deliberate (the slot is *"owed
    its affordance"*, `slot_value_ui`), and the text door admits the
    slot.
  - A typed `inf` in the probe field is refused `NonRigidFrame`. That
    is a refusal of a typed value, not a gate, and `props::field_edit`
    reads non-finite text as a number on purpose.
  - The eight `add_enabled` sites edit drafts only.
  - `delete_button` on a node selection: `present` reads the committed
    document outside a gesture.
- `pane/features.rs`: the row label and the failure-line links push
  `Select`, which never refuses.
- `tree.rs`: it draws no control. It is the row model.

**Blind spots, and the second pass on each.**

- *Late refusals* (named). The second pass found two members that pass
  only through it: the face delete (`EditError::UnknownNode` at
  `commit`) and Add profile's program check. What it cannot see is a
  late refusal the chrome has no value for (`probe_bounds` after its
  gate, a part file whose header scans but whose load fails).
- *A fifth op-pushing helper* (named). There was one when the row was
  written: `widgets::value_field_ops` pushes
  `SetSlot`/`SetSlotExpression`/`SetParam`/`SetParamText` and the
  gesture ops. Outside `widgets.rs`, two more push through a closure or
  a tool: `ViewerBehavior::tool_commit_row` and `blend_commit_row` in
  `pane/create.rs`. Both are in the member list. Instrument change 1
  above is the re-derivation. It still cannot see an op sent some way
  other than `ops.push`.
- *A gate agreeing with its door by coincidence.* This is where the
  landed-versus-committed members were. The pass looked at every gate
  that reads `landed_pair()`, `standing()`, `tree_rows()` or a scan
  snapshot, and it covered the six named files only. `pane/viewport.rs`
  and `pane/view.rs` were not ground here.

**What the instrument above got wrong.**

- "Swept in `properties.rs`" was not complete. The face delete is
  there. It is invisible to an admission-test comparison, and to a
  sweep of `delete_button`'s body, because the gate lives in the
  caller.
- "The four in `widgets.rs`" missed `value_field_ops`, which already
  existed. Two more op-pushing helpers live in `pane/create.rs`.
- `pane/profile.rs` has five `add_enabled` sites, not eight.
  `ShapeEdits::free()` is read at two of them. The rest of the `free()`
  gates are in `widgets.rs`, and they edit drafts only.
- `tree.rs` holds no row controls, and the feature tree has no delete.
  `DeleteNode` is pushed only from `pane/properties.rs`.
- "The one named instance" (Add profile's unreachable arm) is still a
  non-member: that arm is the form's own `(None, _)` lowering. Add
  profile is a member through the door's frame check and through its
  absent preview.
