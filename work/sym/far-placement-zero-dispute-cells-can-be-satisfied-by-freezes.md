---
id: far-placement-zero-dispute-cells-can-be-satisfied-by-freezes
kind: issue
title: The far-placement table's zero-dispute cells at Sym<f64> can be satisfied by freezes
status: open
opened: 2026-10-13
priority: P4
cost: E
---



Found by the sweep of `work/nurbs/disp-zero-can-be-satisfied-by-freezes`
(the dispute-coverage PR). Filed here because the dispute column and
the SYM-11 table are this program's; the test file's territory is
tcost/tint.

`crates/sweep/tests/sym11_far_placement_rows.rs` asserts each cell's
dispute count at `Sym<f64>` (`assert_lane`, `TABLE`): three cells of
the table expect 3, every other inexact cell expects 0. A positive cell
goes red if its checks freeze, because the count falls. A zero cell does
not: a contradiction check over a frozen form cannot dispute, so those
cells pass whether their checks saw their forms or not. That is the
blind spot the thread-count digest had.

The fix is the same one: `SymCounts` now splits the checks into
`checks_expanded` and `checks_frozen`. The thread-count digest guards
on the share that saw a form with no frozen node
(`dispute_coverage` in
`crates/sweep/tests/mass_props_are_thread_count_invariant.rs`). Assert
the same floor, or `checks_frozen == 0`, on the zero cells; at
`max_terms: 4096` the second may simply hold. Measure first.
