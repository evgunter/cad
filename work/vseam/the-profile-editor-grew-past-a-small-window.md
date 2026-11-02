---
id: the-profile-editor-grew-past-a-small-window
kind: issue
title: The profile editor's per-row argument doors push Apply below a 600 px window
status: open
opened: 2026-10-31
priority: P3
cost: E
refs: [6144]
---


## The finding

From the review of PR 6144 (N1). Since that PR, each kept step row of the
profile editor (`pane::profile` `edit_door_ui`) carries a unit picker per
quantity, a `range … name …` line under the row
(`pane::properties` `ViewerBehavior::held_slot_doors_ui`) and the slot's
notes. The startup plate's editor, with five outline rows and the hole, now
pushes Apply below a 600 px window: the reviewer saw it fall off screen. A
whole-app row that clicks Apply has to use the 1000 px harness window.

Nothing here is wrong, but the editor is much taller than it was. Possible
shapes are to fold a row's door line until its row is hovered or focused,
to keep Apply and Revert pinned above the list, or to put the range and
name doors in the field's context menu. Which one is a layout call that
PR 6144 did not make.
