---
id: a-ring-on-a-cone-wall-escalates-props-chart-loop-whole-at-1e-12
kind: issue
title: Main red at ε 1e-12: reach_cone_root_lane's ring on a cone wall whose winding sums to rounding escalates props_chart_loop_whole
status: open
opened: 2026-10-19
priority: P1
cost: M
---


Found by RESTREAD: PR 5363's CI (run 39237439563, job `test`) went red on it, and the PR changes only test code in `torus_region.rs` and `torus_region_probe.rs`. The orchestrator reproduced it on `origin/main` 20a9d7928f.

**The red.** The test is `sweep::all reach_cone_root_lane::a_ring_on_a_cone_wall_whose_winding_sums_to_rounding_answers` at `CAD_TOLERANCE_EPS=1e-12` (`reach_cone_root_lane.rs`, the `wanted a body` panic). It is the cone-ring fuzz's seed 15: a turned cube through a frustum's wall. `∪` returns `ResultInvalid { VolumeUncomputable { face: FaceKey(3v1), source: Escalated { Indeterminate { margin: Enclosure { lo: 0.0, hi: 2.109e-12 }, band: { zero: 1e-12, escalate: 1e-11 }, predicate: "props_chart_loop_whole" } } } }`. The ring's closure read (`geom_brep::props::closure`, `props_chart_loop_whole`) encloses zero to about 2.1e-12 on the cone face and escalates. The test's doc says the winding was decided against half a turn so that it would not escalate. The closure check now reads the ring's sum against zero.

**Not measured.** Which merge turned it red. The test was last touched by PR 5162 (022dad8974). The 2026-10-19 nightly (run 39205007024) shows `reach_cone_split`'s steep frustum red at 1e-12 (`steep-frustum-operand-refuses-its-own-gate-at-1e-12-since-5038`, this slate), and its log tail did not show this row. So this may have turned red after that nightly. Bisect on main's first-parent line from 20a9d7928f back.

**Owed.** Decide whether a ring whose winding sums to rounding should read as whole at this ε. If it should, fix the closure read. If not, re-pin the row to the refusal, with its reason.
