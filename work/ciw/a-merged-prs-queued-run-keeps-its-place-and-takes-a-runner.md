---
id: a-merged-prs-queued-run-keeps-its-place-and-takes-a-runner
kind: issue
title: A merged PR's queued run keeps its place in the queue and takes a runner in its turn
status: open
opened: 2026-11-23
priority: P0
cost: E
---


`ci.yml`'s concurrency group is `${{ github.workflow }}-${{ github.ref }}`
with `cancel-in-progress`, and its `pull_request` trigger lists
`opened, synchronize, reopened, ready_for_review`. A push to a PR
cancels that PR's earlier run; **nothing cancels a PR's run when the PR
merges or closes**. The run stays queued, and when its turn comes it
takes runners for a head that is already on main.

## Measured (2026-11-23, 03:00Z, from the Actions API)

- 103 runs queued, 92 of them `CI`; 5 runs in progress, each with one
  job on a runner (4 `CI`, 1 `work-status`).
- The five most recent `CI` runs that concluded success or failure
  waited 32 min to 2 h 30 min between the run's creation and their
  first long job's start; the three on PR branches waited 2 h 16 min,
  2 h 23 min and 2 h 30 min.
- Of the last 300 completed `CI` runs, 242 were cancelled.
- Of the seven runs queued on `oneround/` branches, three were on PRs
  already merged (PRs 7203, 7407, 7502). ONEROUND's orchestrator
  cancelled those three by hand. The share across the other 85 queued
  `CI` runs was not read.
- One `CI` run on `main` created 2026-08-06 (31121605266) still lists
  as queued.

Not established here: why five jobs were on runners while 92 runs
waited, against the 20-concurrent-job cap `latency-cut` records. That
is the larger part of the wait and is not this row's fix.

## The fix this row names

Add `closed` to the `pull_request` types, with every job of that event
skipped (the `gate ok` aggregator included, since it runs on
`always()`): the closed event's run joins the PR's concurrency group,
cancels the queued run, and needs no runner itself. Check on a merged
test PR that the queued run is cancelled and that no job of the closing
run is given a runner.
