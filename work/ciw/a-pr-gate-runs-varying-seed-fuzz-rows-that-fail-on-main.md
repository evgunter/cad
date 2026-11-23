---
id: a-pr-gate-runs-varying-seed-fuzz-rows-that-fail-on-main
kind: issue
title: The PR gate runs varying-seed fuzz rows whose failure rate on main is above zero, so a PR reds on seeds independent of its diff
status: open
opened: 2026-11-21
priority: P2
cost: M
---


Filed by GERM's orchestrator (2026-11-21).

## What

The PR gate runs fuzz rows with varying seeds, among them
`review_d18::torn_bodies_fail_reads_only_on_a_row_four_premise` (CHARTTAIL's
open torn-body row), the window-oracle rows (CPAIR), and
`ellipse_torus::certified_torus_answers_hold_against_the_true_distance`
(SPLICE). Each of these fails on main at a known, nonzero rate. Every push
draws fresh seeds, so a PR can red on a seed that has nothing to do with its
diff, and the lane has to show each red inherited (it reproduces on main) or
regressed (the failure rate is higher on the branch than on main) before
moving on.

## Evidence

GERM's U3a (PR 7226) spent about two hours of one fix pass on such reds on
2026-11-21. U1b (PR 7184), the P0 lane (PR 7183) and the U2 lane hit the
torn-body seed as well. Each seed is filed as evidence on its owner's row.

## Owed

A rule, decided here: either a varying-seed row with a known open defect
runs at a fixed seed in the PR gate (and varies in the nightly), or the
gate tells the lane which open row a failing seed belongs to. A lane should
not have to re-derive "inherited" for every push.

- 2026-11-22 — **Re-banded P3 → P2 by CIW triage.** - Filed 11-21. GERM's U3a (PR 7226) spent about 2 h of one fix pass on
seed reds, and U1b (PR 7184), the P0 lane (PR 7183) and U2 hit the
same torn-body seed.
- Earlier: onecopy's `main-red-the-circle-lane-coefficient-charge-row-fails-on-some-seeds`
(11-09, "on main and every PR's ci").
- The fix (a fixed seed per PR, varying seeds in the nightly) removes
work and adds no cost.
- Not this family's mechanism.
