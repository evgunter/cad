---
id: a-face-or-edge-delete-is-live-on-a-feature-the-committed-document-no-longer-holds
kind: issue
title: The face and edge header's Delete is gated on the landed run's resolution, and delete_node refuses a feature the committed document no longer holds
status: open
opened: 2026-09-28
priority: P3
cost: E
refs: [the-mirror-class-is-unswept-outside-the-properties-pane]
---


Found by the census in `the-mirror-class-is-unswept-outside-the-properties-pane`,
at merge base `f4e9aa68b`. It lies inside `pane/properties.rs`, the one
file PR #2961 swept. That sweep compared gates with door ADMISSION
tests; this refusal arrives at `commit`, which is the census's first
named blind spot, and the chrome can compute it.

## What happens

`ViewerBehavior::entity_header_ui` (`crates/viewer/src/pane/properties.rs`
~:440) draws `delete_button` (`crates/viewer/src/widgets.rs`) for a
face or edge selection when `standing.live()`.
`DocSession::standing` resolves a face or edge through
`entity_resolution`, which asks `landed_pair()`, the last LANDED run.
The door, `DocSession::delete_node`, reads the COMMITTED document:
`cascade_delete_order(committed_doc(), node)` comes back empty for a
node it does not hold, the single `DocEdit::DeleteNode` goes to
`commit`, and the edit door refuses with
`Refusal::Edit(EditError::UnknownNode)`.

The state: a face of feature F is selected, then F leaves the committed
document (this same Delete clicked once, or an undo of F's insertion).
Until the next evaluation lands the face still resolves, so the button
stays live. After `CancelEvaluation` it stays live indefinitely. Its
label also falls to `DeleteAffordance::of`'s id-only arm, "Delete #n"
(`crates/viewer/src/session/delete.rs`), whose doc says *"no button
renders for one today"*. That is false in this window.

The node-selection path (`properties.rs` ~:404) is not a member: its
`present` reads `doc()`, the committed document outside a gesture.

## Fix

Gate the header's button on the committed document holding the feature,
as the node path does: `standing.live() &&
session.committed_doc().node(feature).is_some()`. Then correct the
id-only arm's doc. It is the landed/committed shape of the VSEAM row
`a-held-pick-is-admitted-off-the-landed-run-while-its-door-reads-the-committed-document`.
