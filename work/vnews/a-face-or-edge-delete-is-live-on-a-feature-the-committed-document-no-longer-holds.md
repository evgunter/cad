---
id: a-face-or-edge-delete-is-live-on-a-feature-the-committed-document-no-longer-holds
kind: issue
title: The face and edge header's Delete is gated on the landed run's resolution, and delete_node refuses a feature the committed document no longer holds
status: closed
opened: 2026-09-28
priority: P3
cost: E
refs: [the-mirror-class-is-unswept-outside-the-properties-pane]
closed: 2026-09-28
branch: vnews/a-stale-delete-says-why
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

## Closed (VNEWS, 2026-09-28)

The button is drawn disabled rather than hidden, as its siblings are
(the hide toggle, the unit picker): `DocSession::delete_refusal`
(`crates/viewer/src/session.rs`, beside `delete_node`) walks the op's
own branch over the committed document and asks the edit door for its
verdict, so the refusal is `EditError::UnknownNode`'s sentence and has
one home. `widgets::delete_button` reads it, so both callers carry the
gate and the header's call site is unchanged. The id-only arm's doc in
`DeleteAffordance::of` now says when that arm is drawn.

Pinned by `app::properties_pane_tests`'
`a_stale_faces_delete_is_disabled_with_the_refusal_it_would_get` and
`a_held_faces_delete_is_live_and_says_nothing`, which drive the whole
app over an inline session whose re-evaluation was canceled.

The sweep's one new member, the *"carries no parameters"* line under
the same stale face, is
`a-stale-face-says-its-deleted-feature-carries-no-parameters`.
