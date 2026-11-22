---
id: the-one-second-bar-reds-prs-off-their-path-several-times-a-day
kind: issue
title: The fast set's one-second bar redded five PRs off their path in one evening, each on a different existing row
status: open
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
