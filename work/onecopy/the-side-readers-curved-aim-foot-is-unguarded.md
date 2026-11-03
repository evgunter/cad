---
id: the-side-readers-curved-aim-foot-is-unguarded
kind: issue
title: side_reader_aims_ask_nothing no longer sees a curved aim's foot: curve_foot_branch is unlogged (F31)
status: open
opened: 2026-11-03
priority: P3
cost: E
---


Filed by `onecopy/foot-and-angle` (PR 6449). That PR moved `geom::curves::on_carrier::angle_near`'s half-turn flip (`curve_foot_branch`) onto `k_stats::check_unlogged(.., "F31")`, because no foot rests on its outcome. `crates/sweep/tests/side_reader_aims_ask_nothing.rs` listed `curve_foot_branch` in `WALKS`, the predicates no aimed ray at its bodies may take, as its marker for a curved aim's on-carrier foot. Unlogged, the predicate never reaches the verdict or escalation log the row reads, so the entry was deleted rather than left unable to fire.

What the row no longer sees: an aim at the rod's cylinder wall (or a cone wall) that takes a closed-form circle foot. The remaining entries (`point_in_loop`, `point_in_arc_loop`, `bool_ruled_region`, `bool_pierce_normal_on`) are not shown to fire on that path.

Fix, the cheap way: read the path's samples, which still reach the sink under `curve_foot_branch` (`geom_core::k_stats` under the `probe` feature), and assert none are taken by the aims. Alternatively, name a logged predicate on the foot path the aims must not take.
