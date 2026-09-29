---
id: a-held-pick-is-admitted-off-the-landed-run-while-its-door-reads-the-committed-document
kind: issue
title: Add datum and Add profile admit a held frame or face pick off the landed run; their doors run require_kind on the committed document
status: open
opened: 2026-09-28
priority: P3
cost: E
refs: [the-mirror-class-is-unswept-outside-the-properties-pane, a-creation-forms-held-pick-survives-a-document-swap, a-panels-gate-reads-the-previewed-document-while-its-door-reads-the-committed-one]
---


Found by the census in `the-mirror-class-is-unswept-outside-the-properties-pane`
(VNEWS), at merge base `f4e9aa68b`.

## The class

Three creation-form gates judge a held pick against the LANDED run;
their doors run `DocSession::require_kind` against the COMMITTED
document. The two differ from a commit until its evaluation lands, and
indefinitely after `CancelEvaluation`, which leaves `derived.landed` as
it was.

- **Add datum, "axis in sketch"** (`ViewerBehavior::add_datum_ui`,
  `crates/viewer/src/pane/create.rs` ~:833). Gate: `!unpicked`
  (`drafts.datum_frame` is set). The picker lists `Self::frames`
  (~:921), which reads `landed_pair()`, and `frame_picker` (~:184) keeps
  showing a held pick the list no longer holds. Door: `add_datum`,
  `require_kind(plane, Frame)`. State: the held frame was undone or
  deleted since. Refusal: `WrongNodeKind { wanted: Frame }`.
- **Add datum, "frame on face"** (same button). Gate:
  `face_frame_seat(landed_pair(), ..)` (`session/refuse.rs`), which asks
  the landed evaluation. Door: `require_kind(at, Body)` on the committed
  document. State: the face's node was undone or deleted and the
  undo's evaluation has not landed. Refusal: `WrongNodeKind { wanted:
  Body }`. (The value-versus-kind disagreement on a transform of a
  pattern runs the safe way: the gate is the stricter there.)
- **Add profile on an existing frame** (`add_profile_ui`, ~:1122). Gate:
  `blocked.is_none() && !refused`, where `refused` comes from a preview
  that is `None` for a frame the landed document lacks (see
  `an-absent-profile-preview-reads-as-admitted`). Door: `add_profile`,
  `require_kind(plane, Frame)`. Same state, `WrongNodeKind { wanted:
  Frame }`.

## Fix

Answer each held pick with `admits(committed_doc().node(pick), wanted)`
next to the landed-run gate, and draw the button disabled with the
door's sentence. The properties pane's `instance_ui` argues the same
reading (*"The COMMITTED document, because that is the one the doors
read"*).

## Neighbours

`work/forms/a-creation-forms-held-pick-survives-a-document-swap` is the
aliasing half of the same held pick, where the door ADMITS a different
node of the same id. This row is the refusing half.
`a-panels-gate-reads-the-previewed-document-while-its-door-reads-the-committed-one`
is the same disagreement with the previewed document instead of the
landed one. The VNEWS row
`a-face-or-edge-delete-is-live-on-a-feature-the-committed-document-no-longer-holds`
is this shape in the properties pane.
