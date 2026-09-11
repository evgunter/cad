---
id: a-keyboard-bump-lands-and-closes-the-pointers-own-probe
kind: issue
title: A keyboard bump on a sibling component lands and closes the probe the pointer is still holding
status: open
opened: 2026-09-11
---



Found by the free-move reachability unit (2026-09-11), as the frame
AFTER the one that unit closed its own question on.

The probe field is three `DragValue`s over one instance, each spelling
the whole gesture triple through `drag_ops`
(`crates/viewer/src/pane/properties.rs:387-407`). A `DragValue` enters
keyboard-edit mode the frame it takes focus
(`egui-0.36.1/src/widgets/drag_value.rs:462-466`), so Tab and ArrowUp
change a component while the pointer holds another, and the typed arm
(`crates/viewer/src/widgets.rs:69-73`) emits
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
