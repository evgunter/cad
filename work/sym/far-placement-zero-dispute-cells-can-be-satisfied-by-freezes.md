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
(PR 4944). Filed here because the dispute column and the SYM-11 table
are this program's; the test file's territory is tcost/tint.

`crates/sweep/tests/sym11_far_placement_rows.rs` asserts each cell's
dispute count at `Sym<f64>` (`assert_lane`, `TABLE`). Four inexact
cells expect 3 disputes, one at ε = 1e-9 and three at 1e-12; every
other inexact cell expects 0. A positive cell goes red if its checks
go blind, because the count falls. A zero cell does not: a
contradiction check over a form it cannot see whole cannot dispute, so
those cells pass whether their checks saw their forms or not. The
thread-count digest had the same blind spot.

PR 4944's fix: `SymCounts` splits the checks into `checks_expanded`
and `checks_frozen`. The thread-count digest guards each receipt on
the share of checks that saw a whole form (`dispute_coverage` in
`crates/sweep/tests/mass_props_are_thread_count_invariant.rs`). Assert
the same here on the zero cells. Measure first.

Two things to know before measuring:

- **Values built before the session read frozen.** A check counts as
  expanded only when its form reaches FREE indeterminates alone: π, or
  the parameters, opaque values and hulls of nodes the session
  recorded. A node minted before the session is installed is not in
  its table, so the walk makes it an unknown and the check reads
  frozen. That is what it is: in-session, the digest's bodies give 412
  `sz` on `validate_geometric` where the outside build gave 48.
  - Under the first classifier PR 4944 shipped, which counted only the
    session's own node ids as unknowns, such cells would read expanded:
    a false green on any `checks_frozen == 0` proposed here.
  - Check where this table's bodies are minted relative to
    `with_session_rules` before reading the columns.
- **The `Interval` assert-not-count sites share the coverage gap,
  unmeasured**: `editor-core/tests/m10_9_pins_interval.rs`
  (`theorems_disputed == 0`) and
  `geom-core/tests/sym11_witness_kind_interval_rows.rs`. There a
  contradiction is a `debug_assert!`, and the assert over a form it
  cannot see whole cannot fire either. The same columns count those
  checks (both witness kinds charge them), so the same share applies.
