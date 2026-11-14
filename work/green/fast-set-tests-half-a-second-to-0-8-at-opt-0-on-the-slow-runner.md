---
id: fast-set-tests-half-a-second-to-0-8-at-opt-0-on-the-slow-runner
kind: issue
title: 50 fast-set tests run 0.5-0.8 s on the slow runner with integration binaries at opt 0 (24 of them under 0.5 s at opt 1), one runner-ratio from the 1 s bar
status: open
opened: 2026-11-02
priority: P4
cost: E
---


## Found (GREEN latency lane, 2026-11-02, PR 6368 run 41453582409)

With the PR test job's integration binaries at opt 0
(`scripts/rustc-test-crates-opt0.sh`), the kernel shard on an EPYC 7763
(the slow runner type) ran 59 fast-set tests at or above 0.5 s. 36 tests
ran that slow at opt 1 on the same runner.
- The 9 at or above 0.8 s moved to the slow set in the PR that took the
  wrapper. 8 of them were already 0.80–0.90 s at opt 1.
- **50 remain in 0.5–0.8 s.** For 24 of them, opt 0 is what put them
  there: they ran under 0.5 s at opt 1. They are mostly `geom-core::all`'s
  `m5_pr7b_tensor_compose` / `review_m5_pr7b_tensor` rows (0.13–0.23 →
  0.51–0.73 s), plus `sweep::all`'s `near_circular_sections` and band rows.
- **The flap risk:** the closed
  `work/ciw/fast-set-tests-over-one-second-are-not-enforced.md` saw
  hosted/local ratios up to 2.8, and saw tests at 1.04–1.37 s flap. A
  slower runner than this one, or a noisy neighbour, carries a 0.7 s row
  over the 1 s backstop and reds an unrelated PR.

## Regenerate

PR 6368's temporary step does it: commit `2f724bb84e3` on
`green/kernel-binary-ab2`, and `.github/kba-list.py` there. On one
runner, the step builds the test job both ways from one restored cache,
copies `target/nextest/ci/junit.xml` after each default-eps run, and
lists every test at or above 0.5 s with its opt-1 time beside it.

## Owed

Decide between two remedies. One is to move the 24 that opt 0 put over
0.5 s to the slow set, by the discipline's own 0.5 s rule. The other is
to leave them and watch `check-fast-set-times` reds for them.

## Two crossed the bar on an unrelated PR (PR 6931, run 43619394495, 2026-11-14)

`test (kernel)` on `green/default-eps` at `c602fc639` passed all 7166
default-ε tests and failed the time check on two existing sweep tests
that PR's diff does not touch:

    sweep::all ring_carry_through_by_piece::a_three_arc_ring_is_metered_arc_by_arc_in_every_subtraction_order took 1.042 s (existing test: 1.0 s)
    sweep::all pis_arc_capped_poses::the_cut_cylinder_reads_its_truth took 1.015 s (existing test: 1.0 s)

The run before it on the same branch (43555174396) passed the same
check, so both were under the bar there. The lane's integration
could not re-run the job (`rerun-failed-jobs`: 403), so the only way to
a second reading was a new commit. This is the second remedy's cost
("leave them and watch `check-fast-set-times` reds"): a red gate on a
PR that changed neither test.

The next run on that branch (43621683400, head `fdba8b0ec`, merged with
main `df4aa65a8`) failed the same check on the first of the two again,
1.037 s, with 7172 tests passing and nothing else over the bar. Twice
at 1.04 s is the test standing at the bar, not one slow runner, so PR
6931 moves
`ring_carry_through_by_piece::a_three_arc_ring_is_metered_arc_by_arc_in_every_subtraction_order`
to the slow set, the remedy the check names. Not determined: when it
reached 1 s (it was under the bar at `6ae8d4c99`, before twelve hours
of main were merged in) or whether main's own gate is red on it.
`pis_arc_capped_poses::the_cut_cylinder_reads_its_truth` (1.015 s once,
under the bar the second time) is left in the fast set.

## Instance (PR 6956, `green/goldens`, 2026-11-14)

Run 43629119488 (head `728013904`, merged with main `a03a2471a`) failed `test (kernel)` on the fast-set bar alone, with every nextest row passing: five existing `sweep::all` rows between 1.001 and 1.097 s against the 1.0 s bar, in a diff that touches no kernel source.

- `split_tangent_edge_curved::a_plane_off_a_hole_wall_reads_each_decision_across_the_band` 1.097 s
- `ring_carry_through_by_piece::a_three_arc_ring_is_metered_arc_by_arc_in_every_subtraction_order` 1.049 s
- `pis_arc_capped_poses::the_cut_cylinder_reads_its_truth` 1.035 s
- `parallel_cylinder_join::equal_wider_and_nested_rods_join_along_their_rulings` 1.005 s
- `corner_patch_chart_poles::every_corner_patch_encodes_its_side_and_holds_no_pole_off_a_vertex` 1.001 s

The run before it on the same branch (43618486824, head `0cbea1468`) passed the same step. Five rows over at once by up to 10% reads as one slow runner. The lane could not re-run the job (the integration gets a 403 on `rerun-failed-jobs`), so the next push carried the re-run.
