---
id: main-red-revalue-boss-union-reads-two-period-near-mark-flips-at-1e-6
kind: issue
title: Main red at eps 1e-6: revalue_boss_union reads two period_near_mark sign flips it does not owe
status: open
opened: 2026-11-11
priority: P1
cost: M
---

Found by PCERT's nightly-reds lane (`pcert/nightly-slowset-reds`), triaging nightly 42965378060 (main `b2916558d9`). Filed on tcost, which owns `crates/editor-core/tests/revalue_corpus.rs` (`work.py territory`). No open row names this failure.

**What fails, at ε 1e-6.** `editor-core::all revalue_corpus::revalue_boss_union`:

`boss_union: the decisions the builds answered with different signs are not the owed ones, in order ([]) … flips: [(5987, "period_near_mark", 0.0, 8.88e-16), (5995, "period_near_mark", 0.0, 8.88e-16)]`, `left: ["period_near_mark", "period_near_mark"]`, `right: []`.

**Not new in this window.** The nightly before (42761779270, `50cfcf499d`) failed it with the same two flips, at decisions 5999 and 6007 of 11640. Only the decision count drifts across the window:
- `2e4c44de` and `10e131e7` read 11635 (flips at 5994 and 6002);
- the nightly's head reads 11628 (flips at 5987 and 5995).

So the failure is older than 50cfcf499d. It is unbisected.

Both flips are `geom-core/src/period.rs`'s `period_near_mark` on a margin of exactly `0.0` against `8.9e-16`: a parameter on a period's mark, read on either side of it by the two builds. Decide whether boss_union owes these two flips (and say so in the row's owed list, with the reason) or whether the mark read should not turn on that ulp.
