---
id: a-ring-on-a-cone-wall-escalates-props-chart-loop-whole-at-1e-12
kind: issue
title: Main red at ε 1e-12: reach_cone_root_lane's ring on a cone wall whose winding sums to rounding escalates props_chart_loop_whole
status: closed
pr: 5400
closed: 2026-10-19
opened: 2026-10-19
priority: P1
cost: M
---


Found by RESTREAD: PR 5363's CI (run 39237439563, job `test`) went red on it, and the PR changes only test code in `torus_region.rs` and `torus_region_probe.rs`. The orchestrator reproduced it on `origin/main` 20a9d7928f.

**The red.** The test is `sweep::all reach_cone_root_lane::a_ring_on_a_cone_wall_whose_winding_sums_to_rounding_answers` at `CAD_TOLERANCE_EPS=1e-12` (`reach_cone_root_lane.rs`, the `wanted a body` panic). It is the cone-ring fuzz's seed 15: a turned cube through a frustum's wall. `∪` returns `ResultInvalid { VolumeUncomputable { face: FaceKey(3v1), source: Escalated { Indeterminate { margin: Enclosure { lo: 0.0, hi: 2.109e-12 }, band: { zero: 1e-12, escalate: 1e-11 }, predicate: "props_chart_loop_whole" } } } }`. The ring's closure read (`geom_brep::props::closure`, `props_chart_loop_whole`) encloses zero to about 2.1e-12 on the cone face and escalates. The test's doc says the winding was decided against half a turn so that it would not escalate. The closure check now reads the ring's sum against zero.

**Not measured.** Which merge turned it red. The test was last touched by PR 5162 (022dad8974). The 2026-10-19 nightly (run 39205007024) shows `reach_cone_split`'s steep frustum red at 1e-12 (`steep-frustum-operand-refuses-its-own-gate-at-1e-12-since-5038`, this slate), and its log tail did not show this row. So this may have turned red after that nightly. Bisect on main's first-parent line from 20a9d7928f back.

**Owed.** Decide whether a ring whose winding sums to rounding should read as whole at this ε. If it should, fix the closure read. If not, re-pin the row to the refusal, with its reason.

## Resolution (FLUX, `flux/main-reds-1e-12-cone`)

**Bisected:** red since PR 5333 (`31e6061c`; its first parent passes),
which decided a cone loop's whole turns edge by edge
(`props_chart_loop_whole`, `Σ lever·|miss|` against `3ε`).

**The miss was charging the interval scalar's width, not a
disagreement.** Each edge's miss was read back about the axis against
the very ends its sweep was read off: in real arithmetic every one of
the cone's is zero (a generator's short turn, an ellipse's thirds,
which telescope onto its ends with their branches decided) or a rim's
`O(tilt²)`. At the interval scalar, on bodies whose points are
re-derived to `≈ 1e-13`–`1e-12` (this ring's cube is turned; the steep
frustum is in a generic pose, 410 m out), the read-back enclosed
`≈ 1e-12` per edge: this ring's loop summed to `5.1e-12`, the frustum's
wall to `1.17e-11`, past `3ε` at ε = 1e-12.

**Fix.** A miss is stated where it is exact: `EdgeTurn::exact` (miss
zero) for a cone generator and an ellipse's thirds, and
`EdgeTurn::carried` for a rim, its miss read about its own centre —
the carry to the axis adds to the sweep exactly what it adds to the
ends' turn, and cancels — so it keeps a tilted rim's real term at the
circle's own rounding. The band and the slack are unchanged.

**Rows**, both red on main at 1e-12: `sweep`'s
`reach_cone_root_lane::a_ring_on_a_cone_wall_whose_winding_sums_to_rounding_answers`
(the ellipse part alone reds it) and geom-brep's
`periodic_chart_closure::a_steep_wide_frustum_wall_off_the_origin_measures_at_every_eps`
(the rim and generator parts together red it; each alone stays within
the slack on these bodies).

**What it costs.** The rim's carry (`about_axis`) now moves `W` only
within its half-turn room, so the PR-5333 mutant that drops it
(`a_cone_rim_in_many_arcs_off_the_axis_measures`) no longer reds: its
whole-ness is read in the rim's own frame, where the carry has no part.
