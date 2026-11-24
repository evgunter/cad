---
id: the-one-second-bar-reds-prs-off-their-path-several-times-a-day
kind: issue
title: The fast set's one-second bar redded five PRs off their path in one evening, on four rows none of them touches
status: dispatched
opened: 2026-11-22
priority: P1
cost: M
---


Filed by the EMBED orchestrator, 2026-11-22, with PR 7321.

## What happened

Between 2026-11-21 22:00Z and 2026-11-22 02:25Z, `gate ok` went red on
five EMBED pull requests with every test passing, each time on the
fast-set time bar ("existing test: 1.0 s") and each time on a row the
PR's diff does not reach:

| PR | row | time on the gate |
|---|---|---|
| 7115, 7119, 7120, 7315 | `step-import::all an_edge_is_read_on_its_description_whatever_recognition_says::a_straight_rim_recognition_declines_is_carried_on_its_column` | 1.31 to 1.72 s |
| 7115 | `sweep::all join_pierce_strut_facing::two_out_runs_at_the_corner_build_in_every_op` | 1.00, 1.01 s |
| 7115 | `sweep::all band_turn_overrun_oracles::generic_hexahedra_build_and_fill_at_the_clip_oracle` | 1.07 s |
| 7321 | `sweep::all blend_dual_tangent::the_blend_size_tangent_matches_central_differences` | 1.32 s |

PR 7321 is the change that moves the two sweep rows above to the slow
set. Its own gate went red on a fourth row.

The step-import row was a new test added past the bar without its
slow-set line (PR 7304), which is one cause and has its fix. The three
sweep rows are existing tests that sit at the bar: two reviewers and one
lane timed them the same on main and on their heads (0.89 to 0.97 s for
the step-import row on an idle box; the sweep rows within a few percent
of 1 s), so a runner a few percent slow tips them.

This is the same shape as three rows already here:
`the-ring-lane-interval-row-runs-at-the-one-second-bar-and-reds-a-pr-off-its-path`,
`a-fast-set-germ-row-sits-on-the-one-second-bar-and-reds-a-docs-only-commit`,
`the-snowman-pole-cut-row-sits-on-the-one-second-bar`.

## What it costs

Each red is a CI round trip (10 to 25 minutes on a kernel-seeded diff)
plus a lane's or an orchestrator's attention to show it is not the
PR's. On 2026-11-22 it held the merge of two approved PRs (7115, 7119)
for over an hour and was answered three times by three lanes with the
same one-line config edit, in three PRs that then overlapped.

## What is not decided

Moving a row to the slow set each time one tips is the gate's own
prescription and is what PR 7321 does. It does not stop the next one:
the fast set holds other rows between 0.9 and 1.0 s, and each will tip
on some runner. The candidates, none chosen here:

- sweep the fast set once for every row over some margin under the bar
  (0.8 s on an idle box, say) and move them together;
- make the bar apply as a failure only to tests the diff adds or edits,
  and as a report for existing tests (the nightly then owns the drift);
- time an existing test against its own recorded time rather than a
  fixed second.

The second and third change a standing rule (`docs/prompts/implementer-discipline.md`,
the per-PR gate), so they are GREEN's to propose with this cost in hand.
The first changes nothing but the list.

## Measured (2026-11-22, green/fast-set-margin)

The default-eps step's PASS lines from 40 PR runs (76 jobs of `test
(kernel)` and `test (app)`, the gate's own build and parallelism), the
nightly's opt-2 junit from three runs, and a push-triggered probe that ran
the 330 fast-set tests with gate p90 >= 0.4 s at `-j1` on the gate's build,
twice on each of two runners (run 45050967584, two attempts). Scripts in
`local-scripts/fast-set-times/`, data in `docs/perf-data/fast-set-times/`.

- Each job's median ratio to the tests' medians clusters at about 0.8,
  1.0 and 1.33: three runner classes. Every reading over 1.0 s of a test
  still in the fast set came from a 1.33-1.41 job, except one 2.13 s
  reading of a test that exists only on a PR branch.
- Fast-set tests at each gate p90: 330 at >= 0.4 s, 162 at >= 0.5 s, 87 at
  >= 0.6 s, 47 at >= 0.7 s, 17 at >= 0.8 s, 5 at >= 0.9 s. At `-j1` none
  of the 330 reaches 0.5 s (top 0.458 s). Gate median is 1.73x `-j1` and
  gate p90 2.36x; `-j1` at opt 0 is 1.13x the nightly at opt 2.
- The four that exist on main at p90 >= 0.9 s moved to the slow set as an
  interim (`.config/nextest.toml` says so), which by §2's rule they would
  not: the 0.5 s rule read off gate times would move 162 tests that cost
  0.2-0.46 s alone.

Left open: PR 7397 decides the policy.

- 2026-11-23 (GERM, PR 7553): another row at the bar.
  - **The red:** `topo chord_join::torus_ring_rows::a_ring_on_a_torus_face_is_re_homed_by_a_path` read 1.116 s on PR 7553's hosted run.
  - **The PR's own share:** its exact belly added about 15% (medians of four interleaved local runs: main 2.34 s, the PR before its filter 2.69 s). The PR removed that with a double-double filter (2.30 s).
  - **Still at the bar:** main reads about 0.97 s by the same ratio, so the PR moves the row to the slow set with that measurement.
- 2026-11-24 (GERM, PR 7553): another row at the bar, `sweep::all join_rc_probes::reflex_corner_struts_past_a_half_turn_build_sound`.
  - **On CI:** 1.104 s on PR 7553's hosted run.
  - **Locally,** medians of eight interleaved runs: 2.67 s on main `a7522805e0` and 2.70 s on the PR. That is within noise, so the row sits at the bar on main (about 1.09 s there).
  - **What the PR does:** it moves the row to the slow set with that measurement.
