---
id: the-datum-face-seat-says-a-deleted-node-by-the-selections-snapshot
kind: issue
title: The datum face-frame form's held face says a deleted node through the selection's snapshot, which it outlives
status: open
opened: 2026-10-06
---


Found in `a-selected-node-deleted-is-said-by-tag-where-the-tools-say-its-label`.

`ViewerBehavior::datum_face_frame_rows` (`crates/viewer/src/pane/create.rs`)
says the face the form holds (`Drafts::held_face`, a bare
`FaceSelection`) from the landed document, and a node that document no
longer holds as the selection kept it (`DocSession::selection_said`).
The form keeps its face after the selection moves on ("a tree click
... without losing the pick"), and from then on the selection's snapshot
is of another node: a held face whose minting node is deleted is said
`node <tag>`. Nothing is said wrongly — an id names one node in the
session — but the label is lost exactly where the form outlives the
pick.

**The fix.** `Drafts::datum_face` keeps the face's `SpokenNode`s beside
it when it copies the pick, as `seats.rs`'s seats do, and the row says
the node by them.
