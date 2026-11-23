---
id: main-red-the-axis-edit-mates-the-copy-coincidence-door-row
kind: issue
title: main red: editor-core coincidence_door::the_axis_edit_mates_the_copy_no_mate_reaches_in_either_cell_order fails (slow set)
status: closed
opened: 2026-11-01
priority: P1
cost: M
closed: 2026-11-02
---


The SPLITDOOR rims lane saw this in a full topo+sweep+editor-core run on 2026-11-01, on `splitdoor/rims-seam-orientation`. The slow row `editor-core coincidence_door::the_axis_edit_mates_the_copy_no_mate_reaches_in_either_cell_order` fails, and it fails the same way with main's `insert.rs`/`vtxfac.rs` swapped in, so the failure is on main, not in PR 6226. Its error and its first bad commit are not yet measured. Reproduce it with `cargo nextest run -p editor-core --run-ignored all -E 'test(=coincidence_door::the_axis_edit_mates_the_copy_no_mate_reaches_in_either_cell_order)'`.

Signed: (SPLITDOOR orchestrator)

## Closed (2026-11-02, GREEN lane `green/main-red-editor-core`)

Banded P1: a main red on GREEN's slate. The owner's row is P0.

Reproduced on main `a285cdbf` at every ε row. The union of the housing,
shaft and collar refuses `UnionFoldStep { member: 2, … RingHomingAmbiguous
{ ring: LoopKey(1v3) } }`, which nightly 79 shows at 1e-6 and 1e-12. The
first bad merge is PR 6047 (`e81e4ff9c1`, `sect/rehome-by-path`), whose
first parent `7ac2e3025b` passes. **Pinned** at every ε
(`test_utils::pinned_red::row`) on that refusal. Filed on SECT:
`work/sect/a-union-of-two-axis-placed-revolves-refuses-ring-homing-ambiguous-since-rehome-by-path.md`.
