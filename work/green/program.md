---
id: green
kind: program
title: GREEN — a green main and a fast gate: the infra rows that matter now
status: active
opened: 2026-10-28
group: infra/ci
prefix: green/
tag: (GREEN orchestrator)
paths: [.github/workflows/*, .config/nextest.toml, scripts/ci-filter.py]
keep_out: [opened by a priority-seam cut across the infra group (the user, in chat, 2026-10-28 - most infra tracks carried one or two rows that matter beside a long P3/P4 tail) - the rows moved here by git mv with their ids and bodies, and their old programs keep their tails, CIW owns hosted CI and the workflows as a whole and GREEN shares that ground only for the latency and main-red rows on its slate, so run scripts/work.py territory on a unit branch and announce the seam on CIW's log where a change is coupled, a main red that a kernel program's merge caused is still fixed here when it sits in a row on this slate but the kernel program is told on its log]
---
**Main is green and the per-PR gate is fast.** Every other track's
work goes through the PR gate and lands on main, so two things here
cost everyone: a gate that takes 23 minutes at the median against
a 15-minute bar (`memories/orchestration-model.md`, "Friction is a
finding"), with the nightly's latency alarm red; and a nightly that
has been red on main for a week, so a new regression arrives unseen
among the old ones.

The slate is the latency work (P0), the main reds on infra ground
(P1), and the cheap rows that make the gate untrustworthy: flaky
guards that turn unrelated PRs red, tests red on main under a
configuration no row runs, gate self-tests no job runs.

Charter and order: `work/green/plan.md`; narrative in `work/green/log.md`.
