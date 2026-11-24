---
id: a-step-import-row-runs-at-1-65-s-and-two-sweep-rows-at-the-bar-and-red-a-pr-off-their-path
kind: issue
title: A step-import fast-set row runs at 1.65 s on the PR gate and two sweep rows sit on the one-second bar, so the gate goes red on a PR that touches none of them
status: closed
opened: 2026-11-22
closed: 2026-11-22
priority: P1
---


## Finding

From EMBED lane pole (PR 7115, 2026-11-22). The PR's diff is
`crates/topo/src/injective.rs`, tests of `sweep` and `geom-core`, and
tracker files.

On head `ed4b577b3` (main `b99f71c3d` merged in), CI run 44965954001,
`test (kernel)` failed twice, on the first attempt and on a re-run of
the failed job, with every test passing (11,904 at the default ε and
3,909 at each of 1e-6 and 1e-12). The step that failed is the fast-set
time bar ("existing test: 1.0 s"):

| row | attempt 1 | attempt 2 |
|---|---|---|
| `step-import::all an_edge_is_read_on_its_description_whatever_recognition_says::a_straight_rim_recognition_declines_is_carried_on_its_column` | 1.619 s | 1.651 s |
| `sweep::all join_pierce_strut_facing::two_out_runs_at_the_corner_build_in_every_op` | 1.012 s | 1.004 s |
| `sweep::all band_turn_overrun_oracles::generic_hexahedra_build_and_fill_at_the_clip_oracle` | under the bar | 1.074 s |

The run on the PR's previous main (`94b6b284e`, run 44959407067) was
green, so the step-import row crossed the bar on a commit between the
two mains; which one is not traced.

**Not the PR's change:** on one box, under the `ci` profile, three runs
each, the rows take 3.76 to 3.89 s, 2.10 to 2.23 s and 1.69 to 1.86 s
on the PR's head and 3.84 to 3.96 s, 2.11 to 2.17 s and 1.76 to 1.91 s
on main `b99f71c3d`.

## What is needed

The step-import row is 60% past the bar on both attempts, so it is
slow, not noisy: make it faster or move it to the slow set. The two
sweep rows sit on the bar and trip it by noise.

## Closed (2026-11-22)

Answered by two PRs that move the rows to the slow set under the
0.5 s rule: PR 7120 for the step-import row (on main at `55e8566eeb`),
and PR 7321 (`embed/step-import-bar`) for the two sweep rows.
