---
id: preview-and-commit-carry-no-gesture-identity
kind: issue
title: PreviewGesture and CommitGesture name no gesture, so a field that is refused a begin still drives the open one
status: open
opened: 2026-09-11
refs: [gesture-drags-have-no-cancel-door]
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
