---
id: a-stale-face-says-its-deleted-feature-carries-no-parameters
kind: issue
title: A face of a feature the committed document no longer holds is said to carry no parameters
status: open
opened: 2026-09-28
priority: P3
cost: E
refs: [a-face-or-edge-delete-is-live-on-a-feature-the-committed-document-no-longer-holds]
---

Found by the sweep in
`a-face-or-edge-delete-is-live-on-a-feature-the-committed-document-no-longer-holds`,
which gated the face and edge header's Delete on the committed document.
This is the same landed/committed shape one line down, on a message
rather than a control, so it was left to its own row.

## What happens

`ViewerBehavior`'s properties pane (`crates/viewer/src/pane/properties.rs`,
the `Selection::Face(_) | Selection::Edge(_)` arm, ~:62) draws *"this
feature carries no parameters"* when `groups.is_empty() &&
standing.live()`. For a face or edge, `standing.live()` is resolution
against the last LANDED run (`DocSession::standing` →
`entity_resolution` → `landed_pair()`). `groups` comes from
`DocSession::slot_rows`, which reads `self.doc()` — the committed
document outside a gesture — and `props::slot_rows` returns nothing
for a node that document does not hold.

So in the window the fixed row names (a face of feature F selected, F
deleted or its insertion undone, the next run not yet landed, or
cancelled), the face still resolves, no rows come back, and the pane
says F carries no parameters. It carries nothing because it is not
there. The node path already has this guard:
`app::properties_pane_tests::a_deleted_node_is_not_said_to_carry_no_parameters`
pins it, because a node's `present` reads the committed document.

The fixture that reaches it is
`app::properties_pane_tests::a_face_of_the_body(true)`.

## Fix

Gate the face and edge arm's line on the committed document holding
the feature as well as on resolution, or have the standing line say the
feature is deleted there as the node path does. Which one is the
standing vocabulary's call (`standing_verdict`); the second is the one
that also tells the reader why the Delete beside it is disabled.
