---
id: main-red-msolve2-member-chain-three-levels-deep-seat-sits-at-the-one-second-budget
kind: issue
title: Main red (time budget): editor-core's msolve2_member_chain::a1c_three_levels_deep_seat doubled to about a second and trips the 1.0 s budget
status: open
opened: 2026-11-23
priority: P1
cost: E
---


Found by SCALAR on PR 7291's gate (run 45251608677, head `e85d43b53f`, job `test (app)`): the fast-set time check reports

    editor-core::all msolve2_member_chain::a1c_three_levels_deep_seat took 1.180 s (existing test: 1.0 s)

and the same run's other two ε passes took 0.971 s and 0.990 s, so the row sits on the budget and any slower runner reddens the gate.

It is not that PR's cost. The same PR's code before it merged main (run 45239078892, head `1d2c808534`) ran the row in 0.482 s, 0.482 s and 0.503 s, and its earlier heads in 0.44 to 0.52 s (runs 45205013229, 45191136715). An unrelated branch that carries the same main (run 45254555300) runs it in 0.809 s. So something that reached main between 18:30Z and 19:40Z on 2026-11-23 roughly doubled this row; PR 7291's diff in `editor-core` is a number token's parse in `parse.rs` and `expr.rs`, which the earlier 0.48 s runs already carried.

Not traced: which merge on main did it (candidates in that window touch `editor-core/src`: PR 7426's subtract tool seats and PR 7576's spoken causes), and whether the row is slower for a reason worth fixing or should move to the slow set in `.config/nextest.toml`, which is the check's own suggested remedy.
