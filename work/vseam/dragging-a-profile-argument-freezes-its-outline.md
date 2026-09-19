---
id: dragging-a-profile-argument-freezes-its-outline
kind: issue
title: Dragging a profile argument in the folded argument rows freezes the profile's outline
status: open
opened: 2026-09-19
---


## The finding

Filed by the orchestrator from the delta review of PR 2862 (MINOR m3,
likely, not seen on screen).

To reproduce, select a committed profile and drag one of its numbers
in the folded "arguments" slot rows (`pane::properties::feature_rows_ui`).
While the drag is live, the landed document is the drag's scratch
document, and `Drafts::sync_profile_edit` reads `committed_doc()`. So:

- the held draft and its edit preview keep the pre-drag numbers;
- `Drafts::edited_in_place` still hides the node's committed drawing
  from `sketch::committed`;
- the profile's outline stays frozen at the old value, while features
  built on it follow the drag.

The code is in `app.rs`, where the edit preview and `except` are taken
at the top of the frame.

## What a fix has to decide

Two readings:

- the draft syncs from the landed (scratch) document while a drag is
  live;
- `edited_in_place` yields `None` while a slot-row drag is live, so the
  committed pass draws the dragged state.

Either way, a headless row should show the outline following a live
drag. The per-field row
(`the-profile-editors-fields-carry-no-per-argument-affordances`) would
remove the second door altogether.
