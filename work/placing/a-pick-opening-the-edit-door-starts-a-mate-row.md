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

## Not built (2026-11-14, `placing/p3-editor-batch`)

Returned undecided: the row is a design fork ("Owed (design,
light)"), and no ruling picks a side. The code today is
`MateTool::pick`, whose `Opening::Edit` arm runs `reload` and then
`add_mate`. The two doors beside it open without a row: the tree's
`edit placement`, and the Transform tool's `MateTool::moving`
(`crates/viewer/README.md`: "the placement editor opened on its pick
with no mate row"). The README also says "the editor's own first pick
takes the same route (`matetool::door_on`)". That is true of which
placement the pick opens, but not of the row. Either answer moves
that paragraph and R2's
`r2_a_placed_copy_reopens_through_a_pick_and_its_untouched_draft_stands`,
which strikes the fresh row by hand.
