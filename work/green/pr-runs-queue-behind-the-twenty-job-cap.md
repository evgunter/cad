---
id: pr-runs-queue-behind-the-twenty-job-cap
kind: issue
title: Half the PR gate's median wall-clock is queue: the 20-job cap was saturated 00-12Z on 11-21, and the latency alarm counts the queue the Friction bar does not
status: open
opened: 2026-11-21
priority: P2
cost: M
---


## Found (GREEN PR-latency lane, 2026-11-21)

Measured from the jobs API of every completed (success/failure) `pull_request` run of ci.yml in two windows: A = 11-15..11-16 12Z (n=106), B = 11-20 12Z..11-21 (n=400). "Active" is change filter + the longest middle job + gate ok, i.e. the run with its queue removed; "queue" is the alarm's wall-clock less that.

| window | wall median / p75 / p90 | active median | queue median / p75 / p90 |
|---|---|---|---|
| A (11-15) | 15.7 / 23.2 / 30.8 | 15.4 | 0.1 / 0.1 / 0.4 |
| B (11-20/21) | 26.7 / 39.7 / 59.3 | 17.1 | 7.8 / 20.4 / 36.8 |

Per-job run times barely moved between the windows (lint 11.1 → 11.7, test shards 12.8–13.9 → 12.8, python 28.2 → 28.2; mesh-budget 8.9 → 12.1 is the one that grew). The queue is what moved, and it is the 20-concurrent-job cap: on 11-21 the jobs in progress sat at ~20 with 20–30 more queued every hour from 00Z to 12Z, and the queue cleared after 12Z.

Demand, 11-21, all workflows, job-minutes (21,800 total):

- CI pull_request success 35%, failure 23%, **cancelled 23%** (5,000 job-min spent on runs a later push superseded);
- CI workflow_dispatch 8%; nightly dispatches and schedule 8%; CI push (cache-prime) 2%.
- By job: lint 17%, the two test shards 26%, python 12%, mesh-budget 12%, demos 7%, corrupt input 7%.

PR runs per day rose from 90 (11-15) to 200–213 (11-20, 11-21).

**The alarm counts the queue; the bar does not.** `nightly.yml`'s `ci-latency` takes `updated_at − created_at` ("queue included"), while `memories/orchestration-model.md` (Friction) says time queued for a runner counts against neither bar and is a finding of its own. On B the alarm reads 26.7 while the bar's own quantity is 17.1.

## Options (not chosen; the first is a question about what the alarm means)

- Have the alarm print and judge run time and queue separately (active median against the 15-min bar, queue median as its own line).
- Cut demand: `green/pr-latency-2` takes about 13% of 11-21's PR job-minutes (10% of all workflows'), measured as python's job 28 → 15.3 min and mesh-budget's 12 → 7.5 on its own run 44989289861; the cancelled quarter is the next-largest single share, and it is spent by pushes to ready PRs while their run is in flight.
- More runners (a paid plan) is Ev's call.

A tier-`all` run in the night window, 11-22 (run 44989289861, `green/pr-latency-2`): 110 min wall against about 22 min active. `test (kernel)` queued 60 min, `viewer` 56, `tools` 46, and `gate ok`, a six-second job, 29. At 06:30Z the repo had 20 jobs in progress and 76 queued.

GREEN's own runs, separated as the 11-15 sitting did: window B holds no `green/*` run (382 of 382 completed runs are other programs'; median 28.0 wall, 16.9 active), and window A holds 6 (without them: 15.3 wall, 14.9 active). GREEN's CI-first bisect branches (`green/<lane>-bisect-<sha8>`) will add runs from here, so a later reading should drop `green/*` branches from both numbers.

## A reading on 2026-11-23, from BOXFINISH (not GREEN's measurement)

At 02:58 UTC: 106 runs queued and 6 in progress (CI 101, work-status 8,
merge ref 2, nightly 1), from nineteen branch prefixes (embed 14, walks
12, lathe 11, boxfinish 11, main 10, ciw 10, oneround 8, the rest 5 or
fewer). The oldest current queued run was created 22:47 UTC on 11-22;
the oldest in progress 23:42.

Of the three oldest runs in progress, each had exactly ONE job
executing and none waiting: a run's jobs started one at a time, 5 to 70
minutes apart, each after waiting one to two hours from creation (one
run: `change filter` 58 minutes after creation, then `viewer` at +2 h
13 m, `test (app)` +2 h 30 m, `python` +2 h 36 m, `lint` +3 h). With
six runs in progress that is about six jobs executing, not twenty. I
did not read every in-progress run's jobs, and I cannot see the
account's concurrency or billing.

A lane's PR opened at 01:52 had not had its `change filter` job start
by 02:56.

The question whether a reviewed PR may merge on a locally run gate
while the queue is this long is with Ev on `[ev]` PR 7508.

(BOXFINISH orchestrator)
