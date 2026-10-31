---
id: dragging-a-profile-argument-freezes-its-outline
kind: issue
title: Dragging a profile argument in the folded argument rows freezes the profile's outline
status: closed
opened: 2026-09-19
closed: 2026-10-31
priority: P1
cost: E
pr: 6144
branch: vseam/the-profile-editors-fields-carry-their-arguments
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

## 2026-10-21 — re-banded P0 → P1

A preview defect for the duration of a drag. Nothing is committed wrong,
and the row was filed as likely rather than seen on screen. P1's GUI
band, not P0's.

## 2026-10-31 — closed (PR 6144): the second door is gone

Retired by `the-profile-editors-fields-carry-no-per-argument-affordances`:
the folded slot rows under the editor no longer exist. The editor's own field
moves the held draft every frame of a drag, and the outline is drawn from
that draft (`app::properties_pane_tests::profile_editor_fields::a_drag_in_the_editor_moves_the_outline_live`).

Holding driven profiles opened a sibling, and the PR's review found
that the first fix for it was wrong. A driven argument's number was read
once, at load. Nothing reloaded the draft when what the expression reads
moved, because the program text stays the same. So a variable set,
dragged, released or undone left the drawn outline at the old value.

As fixed in the same PR, a driven field is locked, so the draft never
holds a person's number there. Every frame, each driven argument of a
kept step takes its reading from the landed document, a gesture's scratch
included (`ProfileEdit::read_drivers`, from `Drafts::sync_profile_edit`).
The preview is drawn from the draft, so it follows. That holds for an
untouched draft and for one holding edits, during a drag and after it
(`pane::profile::tests::the_drawn_outline_follows_what_a_driven_argument_reads`,
and the review's `app::properties_pane_tests::review_6144` rows).
