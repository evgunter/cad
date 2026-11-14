---
id: main-red-revalue-boss-union-answers-two-period-near-mark-decisions-on-either-side-since-6482
kind: issue
title: Main red at eps 1e-6 and 1e-12: since PR 6482 revalue_boss_union's two builds answer two period_near_mark decisions with different signs, each on a margin of exactly 0
status: open
opened: 2026-11-14
priority: P1
cost: M
---


Found by PCERT's nightly-reds lane on 2026-11-11 and held on GREEN until its merge was named (`work/green/main-red-revalue-boss-union-reads-two-period-near-mark-flips-at-1e-6.md`, closed onto this row); bisected by GREEN's ε 1e-6 lane (`green/eps-1e-6`, PR 6934).

**What fails**, at ε 1e-6 and at 1e-12, green at the default ε. `CAD_TOLERANCE_EPS=1e-6 cargo nextest run -p editor-core --test all -E 'test(=revalue_corpus::revalue_boss_union)'`, on main `b363f5fa1`:

> boss_union: the decisions the builds answered with different signs are not the owed ones, in order ([]) … flips: [(5609, "period_near_mark", 0.0, 8.881784197001252e-16), (5617, "period_near_mark", 0.0, 8.881784197001252e-16)], mode: Some(InStep) … left: ["period_near_mark", "period_near_mark"], right: []

Both flips are `geom-core/src/period.rs`' `period_near_mark` on a margin of exactly `0.0` against `8.9e-16`. The two decisions' indices drift from night to night (5999 and 6007 of 11640 on 11-10; 5609 and 5617 of 11130 now); the flips do not.

**Bisected, first-parent, at ε 1e-6:** green at `7072c784b`, `477efb442`, `4f80e4eb5` and `e9eaae651`; red at `a6f0b5e99`, the merge of PR 6482 (`onecopy/angle-reads`), whose first parent is `e9eaae651`; red at `1f7642056` and main. The nightly agrees: it passed on 11-04 (main `63cbd7c3`) and has failed since 11-05 (main `ccaa9e0b`).

**Owed.** Decide whether `boss_union` owes these two flips, and name them in the row's owed list with the reason, or whether a parameter standing on a period's mark should not turn on that last place. Why the default ε does not show them is part of the answer.

**Pinned** (PR 6934): `revalue_boss_union` is a standalone test under `test_utils::pinned_red::row` at 1e-6 and 1e-12, on the two flips with their margins and without their indices. Take the pin off with the decision.
