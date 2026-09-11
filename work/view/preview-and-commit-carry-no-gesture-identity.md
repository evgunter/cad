---
id: preview-and-commit-carry-no-gesture-identity
kind: issue
title: PreviewGesture and CommitGesture name no gesture, so a field that is refused a begin still drives the open one
status: closed
opened: 2026-09-11
refs: [gesture-drags-have-no-cancel-door]
closed: 2026-09-11
branch: view/gesture-identity
---


Found by the cancel-door unit (2026-09-11) while tracing what a
stranded value gesture costs. Disclosed as residue there because the
cancel door is a way OUT of the state and this is what the state does
while you are in it — a different defect with a different repair.

## What happens

`SessionOp::PreviewGesture { value }` and `SessionOp::CommitGesture`
carry no gesture identity. `DocSession::preview_gesture` writes `value`
into the node and slot the OPEN gesture stored at its begin
(`crates/viewer/src/session.rs:1462-1502`), and both operations are
permitted mid-gesture (`SessionOp::permitted_during_value_gesture`,
the ops that drive the gesture and would deadlock behind a guard).
`BeginGesture` is not permitted.

So with a gesture already open, a drag on **any other field** emits the
`drag_gesture_ops` triple and gets: the begin refused
`Refusal::GestureInFlight`, the preview applied to the OPEN gesture's
slot with the new field's number, and the commit landing it. Observed
against the tree at `8cf86ec32` + the cancel-door branch, with a drag
open on an extrude's `Distance` and the second drag on a datum's
`Origin(X)`:

```
second begin: Some(GestureInFlight)
preview: previewed [SetParam { node: RecipeNodeId(2), slot: Distance,
                               expr: 0.012 m }]
commit:  committed [SetParam { node: RecipeNodeId(2), slot: Distance,
                               expr: 0.012 m }]
```

A user dragging the datum's x sees the extrude's distance move. The
refusal on the begin is reported, so nothing is silent at the op layer;
what is silent is that the next two ops of the same gesture went
somewhere else.

## Why it matters, and what it is NOT

**It needs a gesture already open when a second drag starts**, and with
one pointer on one `DragValue` that is the stranded state
`gesture-drags-have-no-cancel-door` traced: the drag's own preview lands
an evaluation its picked face does not survive, the panel is handed no
rows, the release is reported by nothing, and the gesture stays open.
That path now has a door — but the door has to be CLICKED, and the
natural thing a reader does when the panel comes back is drag a field.

So this is not fixed by the cancel door and is not an argument against
it. It is the same shape one level in: an operation whose subject is
whatever happens to be open rather than what the caller meant.

## Candidate shapes, each with a cost

- **Carry the identity**: `PreviewGesture { node, slot, value }` and a
  `CommitGesture { node, slot }`, refused when they do not name the open
  gesture. Makes the mismatch typed, and makes the two ops wider; the
  free-move quartet has the same question (`PreviewFreeMove` names a
  frame and no instance).
- **A gesture token** minted by the begin and quoted by the other three
  — narrower payload, one more value in the chrome's hands.
- **Refuse the whole triple when the begin was refused**, in the
  chrome: cheapest, and it puts a rule in the widget helper rather than
  in the vocabulary, which is where `drag_ops` already argues nothing
  should live.

None is obviously right, which is why this is an issue and not a unit.

## Home

VIEW's: `crates/viewer/src/session.rs`,
`crates/viewer/src/session/op.rs`, `crates/viewer/src/widgets.rs`.

## Closed (2026-09-11)

**Shape 1, both doors of it, and the free-move quartet with it.**
`PreviewGesture { node, slot, value }`, `CommitGesture { node, slot }`,
`PreviewParamGesture { name, value }`, `CommitParamGesture { name }`,
`PreviewFreeMove { instance, frame }` and
`CommitFreeMove { instance }` each name the gesture they drive and are
refused when that is not the one in flight — `Refusal::WrongGesture`
for the value drag, `DisplayFault::WrongFreeMove` for the probe. The
two cancels keep naming nothing: their subject is the state, which is
what the chrome's cancel doors are for. The argument is
`crates/viewer/README.md`'s **A driving operation names its own
gesture**.

### What costing the three shapes against the tree changed

**Shape 1 as this item writes it cannot be spelled.**
`PreviewGesture { node, slot, value }` has nothing to say for a
document-parameter drag, which is the other half of the same gesture
(`SessionOp::BeginParamGesture`). So it is either a union payload —
a second spelling of a target vocabulary the two begins already have —
or one preview and one commit per door, which is the tree's own
precedent at `BeginParamGesture` applied to the ops that lacked it.
Two more variants, no new type, no translation at any call site.

**Shape 3 cannot be built where the item puts it.** The chrome pushes
ops into a `Vec` and `ViewerApp::perform_batch` performs them after the
layout walk, so at the moment `widgets::drag_gesture_ops` pushes a
preview no refusal has happened yet — and a begin and its first preview
reach the same batch (`Refusal::rank`'s own worked example). The
cheapest implementable form asks the session which gesture is open
before emitting, which needs the identity made public anyway and then
puts the decision where no other driver of `SessionOp` can reach it.

**Shape 2 is answerable but costs the one recovery the chrome has.**
A token must be minted by the begin, returned through `OpOutcome` and
held per widget between frames; and it refuses the case a target
accepts — the stranded drag's own field, dragged again, which lands the
number the user dragged it to and ends the drag, against the same base
document because nothing that moves the document is permitted mid-drag.
That is the natural recovery this item's own text describes.

### Reachability, re-checked on today's tree

The route holds. `gesture_table.rs`'s
`a_drags_own_preview_can_strand_it_and_the_door_closes_it` passes at
`dba1afd053` (`cargo nextest run -p viewer --features app --test all
-E 'test(a_drags_own_preview_can_strand_it_and_the_door_closes_it)'`,
1 passed), and its strand half is now a helper two rows share.
`the_field_dragged_after_a_strand_does_not_land_in_the_stranded_slot`
continues from it into the second drag — the item's observation, end to
end through the ops the widget emits.

The free-move half is reached through the operation vocabulary and not
through the chrome: no route to a second probe under an open one has
been traced, which is
`free-move-in-flight-refusal-has-no-reachable-producer`'s open
question and is not answered here. `DisplayFault::WrongFreeMove` is a
second arm with the same standing as `FreeMoveInFlight` — that item's
subject is unchanged and its population is now two.

### Residue

- `the-two-drags-name-their-gestures-in-two-shapes` — the value drag
  names a `(node, slot)` or a name and the probe names an instance, so
  `widgets::drag_ops`' two vocabularies are now two shapes of one
  concept with no shared spelling.
