---
id: main-red-the-rounded-pad-reads-24-more-symbolic-decisions-since-planar-foot
kind: issue
title: main red (slow set): m10_9_no_registrant_lies_on_any_measured_document reads r2_rounded_pad at 1684 symbolic against 1660 since PR 6594 (onecopy/planar-foot)
status: open
opened: 2026-11-09
priority: P2
cost: E
---


Found by PR 6663's review (the plate ledger re-pin), which bisected it first-parent: `crates/editor-core/tests/m10_9_pins_interval.rs` `m10_9_no_registrant_lies_on_any_measured_document` passes at `9ac029c2` (PR 6594's first parent) and fails at `38099d0e` (PR 6594, `onecopy/planar-foot`):

> r2_rounded_pad: 1684 symbolic, measured 1660: SymCounts { symbolic: 1684, registrations_refused: 0, registrations_contradicted: 0, theorems_disputed: 0, checks_expanded: 1122, checks_frozen: 128, numeric: 1262, retried: 0, frozen: 3637 }

Re-run on main (merged 2026-11-09) at 1e-6, 1e-9 and 1e-12: red at each, with the same reading. The row is in the slow set, so PR 6594's gate did not run it, and its description does not disclose the move. It is the same merge that moved the plate's walk ledger (re-pinned in PR 6663), so it is one more instance of `work/ciw/dependents-slow-sets-skip-a-kernel-pr.md`, counted there.

`work/shell/main-red-three-m10-interval-pins-moved-at-5728.md` lists this row among PR 5728's moves. The row passes at `9ac029c2`, so this is a later move, not that one.

What is owed (ONECOPY, 6594's lane): 24 more decisions on the pad reach the symbolic tier than the measured count pins. Say whether the planar foot's reading is what added them, and if so, re-pin the measured count with that reason; otherwise find what did. No theorem is disputed and no registration refused, so no verdict is known to have moved.

Reproduce: `cargo nextest run -p editor-core --profile ci --ignore-default-filter -E 'test(/m10_9_pins_interval::m10_9_no_registrant_lies_on_any_measured_document/)'`.
