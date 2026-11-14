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

## 2026-11-14: one over the 1 s bar on a PR that does not touch it (PR 6934)

`editor-core::all emit_sphere_cut_in_cap::the_holdings_over_a_cut_in_name_only_their_operands_faces` took 1.11 s in `test (app)`'s default-eps step on PR 6934's head `5a50b4459` (run 43619658550) and failed the fast-set bar there. The PR's diff is test pins in `sweep` and one `editor-core` test file, none of them this one. One sighting: on the PR's next run (43629480473, head `5d21825c2`) `test (app)` was green and the test is not listed, so it stays in the fast set.

That run failed one of the PR's own instead: `sweep::all band_strip_mouth_cups::a_cups_mouth_a_hair_across_the_trimline_crosses_and_one_inside_goes` at 0.535 s, held to the 0.5 s bar because the PR's pin edits it. It is in the slow set from that PR.

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

## Seen flapping (2026-11-14, PR 6932)

PR 6932 (workflow and script changes only) went red in `test (kernel)`'s
default-eps step on run 43622391108 (head `a76a6805`, a merge of main),
on four fast-set tests over the 1 s backstop:

- `sweep::all snowman::a_tilted_slab_against_the_lens_builds_off_the_seam` 1.162 s
- `sweep::all ring_carry_through_by_piece::a_three_arc_ring_is_metered_arc_by_arc_in_every_subtraction_order` 1.039 s
- `sweep::all pis_arc_capped_poses::the_cut_cylinders_ellipse_face_takes_nothing_away` 1.035 s
- `sweep::all pis_arc_capped_poses::the_cut_cylinder_reads_its_truth` 1.006 s

The same step was green on the PR's previous head (run 43617218864, 52
minutes earlier).

## Instance (PR 6956, `green/goldens`, 2026-11-14)

Run 43629119488 (head `728013904`, merged with main `a03a2471a`) failed `test (kernel)` on the fast-set bar alone, with every nextest row passing: five existing `sweep::all` rows between 1.001 and 1.097 s against the 1.0 s bar, in a diff that touches no kernel source.

- `split_tangent_edge_curved::a_plane_off_a_hole_wall_reads_each_decision_across_the_band` 1.097 s
- `ring_carry_through_by_piece::a_three_arc_ring_is_metered_arc_by_arc_in_every_subtraction_order` 1.049 s
- `pis_arc_capped_poses::the_cut_cylinder_reads_its_truth` 1.035 s
- `parallel_cylinder_join::equal_wider_and_nested_rods_join_along_their_rulings` 1.005 s
- `corner_patch_chart_poles::every_corner_patch_encodes_its_side_and_holds_no_pole_off_a_vertex` 1.001 s

The run before it on the same branch (43618486824, head `0cbea1468`) passed the same step. Five rows over at once by up to 10% reads as one slow runner. The lane could not re-run the job (the integration gets a 403 on `rerun-failed-jobs`), so the next push carried the re-run.

The push that carried the re-run (run 43632515561, head `cfacf073a`, main `988589f49`) failed the same step again, on two rows: `pis_arc_capped_poses::the_cut_cylinder_reads_its_truth` at 1.013 s and `snowman::a_tilted_slab_against_the_lens_builds_off_the_seam` at 1.010 s. That is `pis_arc_capped_poses` over the bar three times (1.015 s on PR 6931's branch, 1.035 s and 1.013 s here), which is the test standing at the bar. PR 6956 moves to the slow set the five rows these two runs read at 1 s or over that were not already there (`ring_carry_through_by_piece` was, by PR 6931): `split_tangent_edge_curved::a_plane_off_a_hole_wall_reads_each_decision_across_the_band`, `pis_arc_capped_poses::the_cut_cylinder_reads_its_truth`, `parallel_cylinder_join::equal_wider_and_nested_rods_join_along_their_rulings`, `corner_patch_chart_poles::every_corner_patch_encodes_its_side_and_holds_no_pole_off_a_vertex` and `snowman::a_tilted_slab_against_the_lens_builds_off_the_seam`. Each costs twice the 0.5 s at which a test belongs there (`docs/prompts/implementer-discipline.md` §2). The other four were over once each, so for them this is the rule applied rather than a measured trend.

A third run (43635079896, head `d40ac7071`) failed the step on a row none of the earlier runs flagged: `pis_arc_capped_poses::the_cut_cylinders_ellipse_face_takes_nothing_away` at 1.034 s. PR 6956 moves it too. Three runs in a row on one branch, each over the bar on different rows by 0.1% to 10%, with no kernel source in the diff: the kernel shard's fast set has a band of rows sitting just under 1 s on this runner, and moving them one at a time does not converge. That is this row's subject; the count of rows in the band wants re-measuring, and the per-PR bar is failing diffs that cannot have slowed anything.
