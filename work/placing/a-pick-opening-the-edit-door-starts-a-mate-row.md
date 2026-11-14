---
id: a-pick-opening-the-edit-door-starts-a-mate-row
kind: issue
title: Opening the Edit door by a pick appends a fresh mate row, so the loaded bundle alone has no Apply until it is struck
status: open
opened: 2026-11-02
cost: M
---


Found by PR 6300's dual review (NOTE); filed by `placing-bundle-editor` in the fix pass.

`MateTool::pick` with no door open opens the Edit door on the picked copy and appends a fresh row holding the pick as its `on` seat (`crates/viewer/src/matetool.rs`). That is the start of a new mate. A person who meant to edit the loaded values sees `SeatEmpty` until they strike the row. R2's `r2_a_placed_copy_reopens_through_a_pick_and_its_untouched_draft_stands` strikes it by hand.

**Owed (design, light).** Choose between keeping the pick-starts-a-mate rule (the panel offers *strike* beside the new row) and opening on the loaded bundle alone, with the next pick starting the row. The tree's `edit placement` door already opens without a row.

## Ruled (orchestrator, 2026-11-14; design light, no ratified text)

A pick that opens the Edit door on a copy opens on the loaded bundle
alone, with no fresh mate row, as the tree's `edit placement` and the
Transform tool's `MateTool::moving` already do. The next pick starts a
row.

## Not built: larger than it looks (2026-11-14, `placing/p3-editor-batch`)

The code change is one line: the `Opening::Edit` arm of
`MateTool::pick` returns after `reload`, without `add_mate` and without
seating the opening pick. It was tried on PR 6919's branch.
- **It breaks a lot.** It turned 51 integration tests red, out of 148
  run in 16 files: `mate_tool_flow` 13, `bundle_editor` 9,
  `review_bundle_editor_r2` 7, `review_gui4_r2` 5, `mate_kinds` 5,
  `review_bundle_r1` 3, `review_recheck_6300` 2, `rv_matehead_probes`
  2, and one each in `assembly_walk`, `edit_maintenance`,
  `review_gui4_r1`, `story_assembly` and `tree_badges`. Almost all of
  them open the editor with a pick on the copy and expect that pick
  seated as the first row's `on`, through `asm::seat_picks`, the
  `seated` helpers and the story walks.
- **It changes the person's flow.** "pick on the copy, then on what it
  meets" (`mate_tool_ui`'s prompt) becomes three picks: open, `on`,
  `to`.
- **The rule leaves one case open.** On a bundle that already holds a
  mate, the editor's `active` row is 0. So under today's
  `Change::Pick` arm, the "next pick" re-seats that loaded mate rather
  than starting a row, which is what the tree and Transform doors do
  now too. Either the rule's "the next pick starts a row" also
  changes `Change::Pick`, or it holds only for a bundle with no mates.

Cost M. It needs that case settled, then the helpers and the README
placement-editor paragraph moved together.

