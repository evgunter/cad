---
id: reach-volume-backstop-fails-off-the-default-eps
kind: issue
title: reach_volume_backstop's rows fail at CAD_TOLERANCE_EPS 1e-6 and 1e-12, so main's eps step is red for any diff that seeds all()
status: closed
opened: 2026-10-01
priority: P0
cost: M
closed: 2026-10-01
pr: 3636
branch: reach/backstop-eps-rows
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

## Closed

PR 3636 (`reach/backstop-eps-rows`), the reach-eps lane. **Outcome: the
rows encoded the default ε, and the backstop answers by its own
contract at every ε.** Measured on clean `origin/main` (`c4788c10d`)
with the enclosures printed round by round: a body's last-round
half-width on the oblique rod is ≈ 2.2e-10 m³ of rule remainder,
whatever ε, plus ≈ 0.55·ε m³. A zero margin between two such bodies
meters ≈ 1.8e-11 m: inside the band at 1e-9 and 1e-6, beyond it
(18ε) at 1e-12.

- The fattenings (1e-7, 1e-9, 1e-11, the union's 1e-7 m shortfall,
  the open-sign 22ε) were default-ε displacements. They are now spelled
  in ε, or in the reporting pads where the row's claim is about the
  pads.
- At 1e-12 the right answer's `VolumeUndecided` is the gate's
  documented fail-loud, the same as the scale-10³ row at the default ε.
  The rows pin it through `last_round_decides`.
- `assert_sound`'s absolute 1e-9 m³ became enclosure containment plus
  the midpoint within ε·area. The 4e-3 rad tilt became a sag of
  2000·ε, because the union's coincidence margin is the cap's sag
  (1.73e-6 = 1.7ε at 1e-6, in band).
- The 0.5 rad convergence gap exists only at the default ε. It stays
  pinned there, and both orders are asserted sound elsewhere.
- The mirror's midpoint asymmetry was a default-ε coincidence. It is
  replaced by a three-arc rod whose midpoint differs at every ε.

The floor evidence went to
`work/quad/quadrature-interval-floor-grows-with-the-body-past-the-band`.
