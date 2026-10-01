---
id: reach-volume-backstop-fails-off-the-default-eps
kind: issue
title: reach_volume_backstop's rows fail at CAD_TOLERANCE_EPS 1e-6 and 1e-12, so main's eps step is red for any diff that seeds all()
status: open
opened: 2026-10-01
priority: P1
cost: E
---


Filed by the EDIT orchestrator. The failure surfaced on EDIT's PR 3625
(run 36818237171), whose diff does not touch `sweep` or `topo`. It
reproduces on a clean `origin/main` (`8028b8b56`):
`CAD_TOLERANCE_EPS=1e-6 cargo nextest run -p sweep -E 'test(/reach_volume_backstop/)'`.

**What fails.** `crates/sweep/tests/reach_volume_backstop.rs` (from
PR 3611, `e1426585f`) has six rows red at eps 1e-6 and four at 1e-12.
It fails in several distinct ways:
- `a_boss_at_half_a_radian_measures_in_one_operand_order`: at 1e-6 the
  volume misses the analytic value by more than the fixed `1e-9`
  (`assert_sound`, :51; 4.084479987922257 against 4.084479986133957).
  At 1e-12 the expected refusal does not come (:141).
- `a_tilted_boss_unions_onto_a_box`: the union refuses (:38).
- `a_result_equal_in_truth_but_larger_at_the_midpoint_passes` (:340),
  `a_planted_union_result_short_by_less_than_the_reporting_pads_refuses`
  (:304), `planted_subtraction_results_refuse_down_to_the_stated_resolution`
  (:261, :267) and `an_open_sign_beyond_the_band_at_the_last_round_refuses`
  (:427; `sweep/src/test_support.rs:631` at 1e-12): planted results
  whose verdicts move with eps.

**Why main looked green.** PR 3611's own run was green; its eps step
covers only the crates the diff seeds. A diff whose eps filter is
`all()` runs this suite at both eps values and goes red. So every such
PR is blocked until this is fixed.

**Final state.** Each row states what it holds at every eps the CI
runs. The bounds and the planted margins are derived from the run's
tolerance (`Tol::eps`, the backstop's stated resolution), not fixed at
the default. Where a verdict really does change with eps, the row
names that and pins it per eps.
