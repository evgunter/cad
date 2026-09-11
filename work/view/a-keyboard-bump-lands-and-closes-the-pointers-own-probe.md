---
id: a-keyboard-bump-lands-and-closes-the-pointers-own-probe
kind: issue
title: A keyboard bump on a sibling component lands and closes the probe the pointer is still holding
status: closed
opened: 2026-09-11
closed: 2026-09-11
refs: [probe-identity-stops-at-the-instance]
---



Found by the free-move reachability unit (2026-09-11), as the frame
AFTER the one that unit closed its own question on.

The probe field is three `DragValue`s over one instance, each spelling
the whole gesture triple through `drag_ops`
(`crates/viewer/src/pane/properties.rs:387-407`, at the SHA this was
filed at). A `DragValue` enters keyboard-edit mode the frame it takes
focus (`egui-0.36.1/src/widgets/drag_value.rs:462-466`), so Tab and
ArrowUp change a component while the pointer holds another, and the
typed arm (`crates/viewer/src/widgets.rs:69-73`, likewise) emits
`[BeginFreeMove, PreviewFreeMove, CommitFreeMove]` into the same batch.

All three name the SAME instance, because there is one probe per
instance and all three components drive it. So after the begin is
refused `FreeMoveInFlight` (`crates/viewer/src/display.rs:746-748`):

- `preview_free_move` finds the open gesture's instance equal to the
  one named (`display.rs:778-780`) and **overwrites the pointer drag's
  previewed frame** with the keyboard-typed one;
- `commit_free_move` takes it on the same test (`display.rs:807`) and
  **lands it and closes the gesture**.

`perform_batch` performs every operation of a batch and keeps the
best-ranked refusal, so the user is shown *"finish the free-move
first"* over a probe the same batch has just finished — and the pointer
is still down on a drag that now has no gesture behind it. Its next
preview and its release refuse `NoFreeMove`.

This is a sharper instance of what `gesture-drags-have-no-cancel-door`
is about than the one that item names: not a refusal whose remedy the
user cannot reach, but a refusal describing a state its own batch
destroyed.

The instance check that lets this through is the one `preview_free_move`
and `commit_free_move` document as the protection against *"a drag on a
second instance's field"* — correct for that, and blind to a second
FIELD on the same instance. Whether the fix belongs at the operation
(naming the component, or the gesture, rather than the instance) or at
the chrome (one gesture mapping for the three components rather than
three) is the fork.

## Closed (`view/keyboard-bump`): fixed at the CHROME

**The probe row is now ONE gesture.** `widgets::vec3_row_ops`
(`crates/viewer/src/widgets.rs:168-211`) draws the three millimetre
boxes, unions their responses (`egui::Response`'s `|`, which is
egui's own summary of a row) and calls `drag_ops` exactly once, with
the composed `[f64; 3]` as the gesture's value. `instance_ui` calls it
(`crates/viewer/src/pane/properties.rs:388-412`).

Under the union, `dragged()` means *the pointer is holding this
gesture* rather than *this box*, which is the question the typed arm
(`changed() && !dragged()`) was already asking and getting a per-box
answer to. So a keyboard bump on a sibling box is no longer a typed
entry: it is a change on a frame the gesture is dragged, and it emits
one `PreviewFreeMove` into the open probe. Nothing begins, nothing
commits, and the frame it previews is composed from all three
components AFTER all three are drawn — which also fixes a smaller thing
the per-box mapping had, that each box's preview closed over an array
whose later components had not been laid out yet.

`drag_ops` and `drag_gesture_ops` are generic over the gesture's value
type for it (`crates/viewer/src/widgets.rs:93-166`). That is forced
rather than decorative: once the row is one gesture there is no single
box's number to pass, and the free-move vocabulary's closures already
ignored the one they were given.

**Why the chrome and not the operation.** The item offered *name the
component* or *name the gesture* as the other side.

- The component is dead: the op takes any rigid `Frame`, deliberately
  (`crates/viewer/src/session/op.rs:321-335`), and three translation
  boxes are one chrome's decomposition of it.
- The gesture cannot be named from here. `widgets::drag_ops` is handed
  the whole vocabulary as VALUES before any of it is performed, and the
  typed arm literally builds `vec![Begin, Preview, Commit]` — no
  payload in that batch can carry a token the begin returned. A
  client-minted id would work, and then the CHROME is the thing
  deciding which drivers are one gesture, which is this fix with an
  extra field on four operations. The operation side needs the chrome
  side first; it is not an alternative to it.

`SessionOp::PreviewFreeMove`'s *"the identity is one node rather than a
target"* is therefore kept and argued rather than corrected: one node
is the whole identity BECAUSE an instance has one probe, and what makes
that enough is that one chrome gesture drives it.

**What this does NOT close** is a second DRIVER on one instance — the
door still cannot refuse one, and today's chrome simply has none.
`probe-identity-stops-at-the-instance` owns that, and it is filed
rather than disclosed here, because *unreachable from today's chrome*
is exactly the claim
`free-move-in-flight-refusal-has-no-reachable-producer` was filed on.

`DisplayFault::FreeMoveInFlight` keeps a producer: `Open` and
`NewDocument` are the two `false` rows of
`SessionOp::permitted_during_free_move` (`op.rs:875`), raised at
`crates/viewer/src/session.rs:1089-1091`.

Held by `a_keyboard_bump_steers_the_held_drag_rather_than_beginning_a_second_probe`
in `crates/viewer/src/widgets.rs`'s test module, which reads the
previewed frame rather than the op kinds — the pointer's own component
and the keyboard's are both in it — plus
`a_keyboard_bump_with_no_drag_open_still_spells_the_whole_triple`, so
the rule stays *the typed arm asks whether THIS gesture is open* rather
than *delete the typed arm*. Measured by mutation: with
`vec3_row_ops` handing `drag_ops` the box that changed instead of the
row, the first row reports `["preview", "begin", "preview", "commit"]`
— this item's defect exactly — and reds.

**One prose universal this made true that was false before**:
`crates/viewer/README.md`'s *"The subject of a driving operation is the
field the user has hold of"* was stated over all six driving ops, and
the probe's three boxes were three fields over one subject. The section
now carries the probe's asymmetry and what makes it enough.
