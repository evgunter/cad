---
id: a-pick-opening-the-edit-door-starts-a-mate-row
kind: issue
title: Opening the Edit door by a pick appends a fresh mate row, so the loaded bundle alone has no Apply until it is struck
status: open
opened: 2026-11-02
---


Found by PR 6300's dual review (NOTE); filed by `placing-bundle-editor` in the fix pass.

`MateTool::pick` with no door open opens the Edit door on the picked copy and appends a fresh row holding the pick as its `on` seat (`crates/viewer/src/matetool.rs`). That is the start of a new mate. A person who meant to edit the loaded values sees `SeatEmpty` until they strike the row. R2's `r2_a_created_copy_reopens_through_a_pick_and_its_untouched_draft_stands` strikes it by hand.

**Owed (design, light).** Choose between keeping the pick-starts-a-mate rule (the panel offers *strike* beside the new row) and opening on the loaded bundle alone, with the next pick starting the row. The tree's `edit placement` door already opens without a row.
