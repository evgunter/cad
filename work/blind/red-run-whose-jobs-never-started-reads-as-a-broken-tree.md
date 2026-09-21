---
id: red-run-whose-jobs-never-started-reads-as-a-broken-tree
kind: issue
title: A run whose jobs were never acquired is indistinguishable from a broken tree, and the tell is undocumented
status: open
opened: 2026-09-15
priority: P3
cost: E
---


## What happened

Run `34951969122` (PR #2630, head `c9e9f861`) reported **FAILED** with all
six `test (interval, …)` jobs red and `gate ok` red downstream. The
default lane's six were green on the same tree, so the red read as
lane-shaped — the exact signature of a real interval-only defect, and it
was triaged as one.

No test ran. Every one of the six carries the check-run annotation:

> The job was not started because it repeatedly failed to be acquired
> (5 attempts).

GitHub could not hand the job to a runner. The tree was never the
subject.

## Why it cost an orchestrator and a lane

Nothing on the run's surface says so:

- the **rollup** says `failure`, the same word a red test gives;
- the **job list** says `failure` on six jobs whose names are the six
  points of one lane, which is what a lane-shaped defect looks like;
- **`get_job_logs` 404s** with `BlobNotFound` — a job that never started
  has no log blob — which reads as "logs expired", and expired logs are
  ordinary;
- the check runs carry **empty `output.text`**, so the obvious second
  place to look is blank;
- the durations (44-57 s) are short enough to look like an early,
  decisive test failure.

Two orchestrator hours and a full lane re-provision (clone + 5.6 GB
target + a queued build) went into reproducing a failure that had not
happened.

## The tell, which is not written down anywhere

**A job that never started reports ZERO steps.** In
`/actions/runs/<id>/jobs`, every job that ran carries its `steps` array
— 3 to 34 entries, with the failing one marked — and all six of these
carried `"steps": []`. That is a one-line discriminator over data the
API returns for free:

```sh
jq -r '.jobs[]|select(.conclusion=="failure")|"\(.steps|length) steps\t\(.name)"'
```

Zero steps ⇒ read the annotation before reading the tree:

```sh
curl -H "Authorization: Bearer $GITHUB_TOKEN" \
     "$(…|jq -r .check_run_url)/annotations"
```

which is where the sentence above lives, and the only place it does.

## What is wanted

A runbook line wherever lanes are told how to read a hosted run — CIW
owns the CI surface and `memories/agent-lane-operations.md` is where the
run-record discipline lives, so the wording is CIW's call and a
`memories/` edit is Ev's. The claim to record is narrow and checkable:
*a red job with zero steps did not run; read its check-run annotation
before you read the diff.*

Not asked for: a retry mechanism. Re-running failed jobs needs a token
scope the lanes do not have (`rerun-failed-jobs` answers `403 Resource
not accessible by integration`), so the practical recovery today is to
push a commit and let a fresh run happen — which is what this row's own
commit did.

Filed by SUITE/D114, from the diagnosis of #2630's red.
