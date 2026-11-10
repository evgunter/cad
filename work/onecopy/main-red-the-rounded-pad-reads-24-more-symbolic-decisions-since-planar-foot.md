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

## Closed

Both moves are real and correct, and are re-pinned (`crates/editor-core/tests/m10_9_pins_interval.rs`, the pad's `symbolic`, `without_the_registry` and the rule-F dials receipt). They were measured by logging each symbolic-tier decision's predicate (temporary logging, not kept) at PR 6594's first parent `9ac029c2`, at its merge `38099d0e`, at PR 6682's first parent and merge, and on main.

**1660 → 1684 (PR 6594, `onecopy/planar-foot`).** Four identical blocks of nine decisions appear, at the pad's four near-tangent line/arc junctions:
- 24 proven: `chord_side` ×2, `line_span`, `circle_side`, `arc_span` and `contact_at_shared_vertex` per block;
- 12 numeric: `nearer_end`, `chord_side` and `circle_side`.

`profile::seg::line_arc` now reads its candidates' height and half-chord through `Line2::height` / `Line2::half_chord`, which divide by the chord direction's norm. Near tangency the half-chord's root widens that division's few ulps, so the candidate's `arc_span` escalates where it had decided. `candidates` then settles it by `end_in_stretch`, whose decisions are the nine.
- Reverting `line_arc`'s arithmetic alone, at `38099d0e`, reads 1660 again.
- The division is the home's ruled form: `Line2::height` is correct for any nonzero direction, and PR 6568 measured the widening at `Interval` as accepted.
- No verdict moved: no theorem is disputed and no registration refused.

**1684 → 1704 (PR 6682, an extension's run is its own segment, Ev's ruling on PR 6542).** It adds 20 symbolic and 106 numeric decisions (`segment_gap`, `carrier_line_circle`, `line_span`, `chord_side`, `collinear_overlap`, `vertex_separation`, …). Each corner after a straight leg keeps the leg's end as a vertex, so the simplicity pass reads more pairs. PR 6682's body names this move and leaves the re-pin to this row.

**One more pin in the file, not this row's move.** `m10_9_what_the_registry_buys` was red on the bracket as well, with the door shut: (1544, 1224) pinned, (1542, 1226) read. It reads (1542, 1226) at `b2eb7192` (PR 6530's merge, which wrote the (1544, 1224) pin with PCERT E1c's re-pin) and at both of that merge's parents. So the pin was never main's reading. It is re-pinned at main's reading, unbisected, with that said at the pin.

With these re-pins, `m10_9_pins_interval` passes 7/7 at 1e-6, 1e-9 and 1e-12.
