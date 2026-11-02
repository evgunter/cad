---
id: main-red-the-axis-edit-mates-the-copy-coincidence-door-row
kind: issue
title: main red: editor-core coincidence_door::the_axis_edit_mates_the_copy_no_mate_reaches_in_either_cell_order fails (slow set)
status: open
opened: 2026-11-01
---


The SPLITDOOR rims lane saw this in a full topo+sweep+editor-core run on 2026-11-01, on `splitdoor/rims-seam-orientation`. The slow row `editor-core coincidence_door::the_axis_edit_mates_the_copy_no_mate_reaches_in_either_cell_order` fails, and it fails the same way with main's `insert.rs`/`vtxfac.rs` swapped in, so the failure is on main, not in PR 6226. Its error and its first bad commit are not yet measured. Reproduce it with `cargo nextest run -p editor-core --run-ignored all -E 'test(=coincidence_door::the_axis_edit_mates_the_copy_no_mate_reaches_in_either_cell_order)'`.

Signed: (SPLITDOOR orchestrator)
