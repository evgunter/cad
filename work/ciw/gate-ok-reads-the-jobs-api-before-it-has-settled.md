---
id: gate-ok-reads-the-jobs-api-before-it-has-settled
kind: issue
title: gate ok reds a green run when the jobs API still reports a finished needs: job as in_progress
status: open
opened: 2026-09-15
---


## Finding

`gate ok` (`.github/workflows/ci.yml`'s `gate-ok` job, whose body is
`scripts/check-run-jobs.py`) reads the run's own job list ONCE and
refuses if any job's `status` is not `completed`:

```python
running = [j for j in others if j.get("status") != "completed"]
```

There is no settle window and no retry. GitHub satisfies a `needs:`
edge before its jobs API necessarily reports that job as `completed`,
so the gate can read a job it genuinely waited for as `in_progress` and
turn a fully green run red.

## The instance

Run **34949708108** (PR 2627, head `54648886f`), 2026-09-15 09:25 UTC.
Every substantive job succeeded — all 12 `test (…)`, all 5
`k-lint (gate, …)`, the python suite, both render lanes, the discipline
and parity rows, freecad import — and the four the change filter skips
were skipped. The final tally on the jobs API is **34 success, 4
skipped, 1 failure**, and the one failure is `gate ok` itself:

```
FAILED: these jobs had not finished when this gate ran:
  k-lint (gate, release-default) (in_progress)
```

`k-lint` IS named in `gate-ok`'s `needs:`, and that job's own
conclusion on the same API is `success`. So the gate did wait for it;
what it read was the API lagging its own scheduler.

## Why it is worth fixing rather than re-running

A lane that meets this cannot re-run: `rerun-failed-jobs` answers
**403 Resource not accessible by integration** for the account these
lanes run as, and so does `workflow_dispatch`. The only remedy is to
push a fresh head, which costs a whole run (~25 min of the shared
runner pool) and puts an empty commit in the branch's history — this
PR's `5f9b77629` and its successor are both that shape.

The gate's own rationale (*"a check that summarises a run it did not
see the end of is the exact thing this job exists to prevent"*) is
right and should not be weakened. What it needs is a bounded settle:
re-read the job list a few times, a few seconds apart, and refuse only
if a job is STILL not `completed` — the false-green the gate guards
against needs the job to stay unfinished, while this false-red needs
only one stale read. `scripts/check-run-jobs.py`'s case 3 selftest
would then pin "still running after the retries", and a new case would
pin "in_progress on the first read, completed on the second".

## Was

Met by SCALAR's VREV fix pass (PR 2627) on run 34949708108.
