---
id: gate-ok-has-no-expected-job-roster
kind: issue
title: check-run-jobs.py holds no expected-job roster: a job absent from the API reads as green on the one required check
status: open
opened: 2026-09-12
priority: P3
cost: E
---


Filed 2026-09-12 by the S-TCOST orchestrator out of the correctness
review of S-TCOST PR 2434, which hit the latent case. `scripts/check-*.py`
is CIW's by `work/ciw/program.md`.

## The gap

`gate ok` is the entire required check: once it is green the run is
mergeable. It runs `scripts/check-run-jobs.py`, whose `check()` builds
its population as **every job in the run except itself** and then tests
three things: paging (`total_count > len(jobs)`), a job not
`completed`, and a conclusion outside the passing set.

**There is no expected-job roster anywhere in the script** — no `needs:`
comparison, no name list, no count pinned to the workflow. So the three
states of a job that should have run resolve as:

| state | verdict |
|---|---|
| `queued` / `in_progress` | **RED** — fail-loud, correct |
| concluded non-green | **RED** — correct |
| **absent from the API response** | **GREEN** |

An absent job is not a decision path the script has. It is simply not in
`others`, the gate prints `gate ok: N jobs, all success, skipped or
neutral`, and returns 0.

**There is no `--selftest` arm for the absent case, and there cannot be
one** while the script holds no expectation of what should exist: there
is nothing for such an arm to assert against. The selftest advertises ten
cases over seven decision paths; an expected-job-missing case is not
among them, and its absence is structural rather than an oversight.

## What currently defends it, and why that is thin

Only `gate-ok`'s own `needs:` list, which names every other job in
`ci.yml`. That is a hand-kept enumeration in a different file from the
check that depends on it, with nothing comparing the two — so the
defence is exactly the shape this repository keeps finding broken.

It broke on PR 2434: that branch restored a job to `ci.yml` and did not
add it to `gate-ok`'s `needs:`, leaving it the only job in the file
outside the list. The hole did not open on that run — the job finished at
+108 s and `gate ok` started at +1145 s — **so it would have merged
green, with the defect intact, had a reviewer not read the `needs:`
list.** The next job added is the next instance.

The script's own failure text says *"This gate runs last because
`needs:` names every other job in ci.yml"* — which states the invariant
precisely, and states it in a comment, in the file that would be the
natural place to enforce it.

## What this asks for

Make the roster executable and put it in one place. The shape that fits
the tree's own standing rule (*a census has one executable home and every
other site points at it*) is for the check to derive the expected set
from `ci.yml` itself — the workflow is in the checkout the job already
does — and red when a job it expected is absent from the run, rather
than trusting a list maintained in a second file.

**Cheaper alternatives, if that is too much:** a selftest arm that parses
`ci.yml`'s job keys and asserts they equal `gate-ok`'s `needs:` would
catch the whole class at lint time and needs no API. That is probably the
right first move, and it is the one that would have caught PR 2434.

## Two things this is not

- Not a claim that any job is currently unguarded: on `main` today
  `gate-ok`'s `needs:` does name every job. This is the absence of
  anything that keeps that true.
- Not the `merge_group` path, which the review did not exercise.

## Related, and the reason this matters more than it looks

`memories/agent-lane-operations.md` records three faces of the
silent-coverage class already — a CONFLICTING PR that gets no run, a run
that queues with zero jobs, and a green job NAME over a SKIPPED step. Its
standing conclusion is *"the run record is the instrument; the workflow
source is not."* This row is the fourth face and the worst-sited: the
instrument reads the run record faithfully and cannot tell a run that had
twenty jobs from one that should have had twenty-one.

## The same population read, in the opposite direction: a FALSE RED (WIRE, 2026-09-13)

Run `34749650737` (WIRE PR 2501, head `bfabc3d7d`). Every job in the run
concluded **success** — twelve `test (…)`, all five `k-lint (gate, …)`,
the python suite, rustdoc, the render lanes. `gate ok` failed anyway:

    FAILED: these jobs had not finished when this gate ran:
      k-lint (gate, release-default) (in_progress)

The timings say what happened. `k-lint (gate, release-default)`
completed at **09:51:02**; `gate ok` started at **09:51:05** and read
that job as `in_progress` at **09:51:11**. `needs:` had been satisfied —
which is what released `gate ok` to start — and the **jobs API was still
serving a snapshot nine seconds stale**.

This is the same root as the row above, read the other way. The script
derives its population and its verdict from one API response and trusts
that response as the state of the run: an absent job reads green, and a
concluded job the API has not caught up on reads red. `needs:` already
carries the "has it finished" answer and is authoritative — the workflow
would not have started this job otherwise — so the `completed` test adds
no information the scheduler did not already give, and costs a false red
whenever the API lags. A roster fixes the green direction; **the red
direction wants the `completed` check to stop being read off the API at
all**, or to be retried until the API agrees with `needs:`.

Cost here: a re-run of the required check on a run that was already
green in every job, plus the reading time to establish that nothing in
the diff was implicated.

## A second false red, two days later (CENSUS, 2026-09-15)

Run `34949096444` (CENSUS PR 2634, head `c306e32b`). Same shape as the
WIRE instance above, same job, and **tighter**:

| event | time |
|---|---|
| `k-lint (gate, release-default)` completed | 09:28:10 |
| `gate ok` started | 09:28:12 |
| `gate ok` read that job as `in_progress` | 09:28:21 |
| `gate ok` failed | 09:28:23 |

39 jobs: **35 success, 3 skipped, 1 failure — and the failure is the gate
itself.** Twelve `test (…)`, all five `k-lint (gate, …)`, the python
suite, rustdoc and every render lane concluded success.

Two things this instance adds to the row:

- **It is not rare and it is not tied to one branch.** Two occurrences in
  three days, both on the same matrix row (`release-default`, the
  longest-running k-lint row at 27 minutes here), both within seconds of
  that row concluding. The window is entered whenever the last job to
  finish is the one that releases `gate ok` — which is the normal case,
  not an unlucky one, so the rate is governed by how often the API lag
  exceeds the gate's own startup time.
- **The gate's diagnostic misdiagnoses it.** The failure text says
  *"a job that is still going is one this gate did not wait for — add it
  to `needs:`"*. `k-lint` **is** in `gate-ok`'s `needs:`, and the WIRE
  instance was the same. So the message sends a reader to check a list
  that is already correct, which is where the reading time named in this
  row's cost paragraph goes. Whatever fixes the red direction should also
  stop the script asserting a cause it has not established: it cannot
  distinguish "you forgot a `needs:` entry" from "the API has not caught
  up with a `needs:` you have", and it states the first as fact.

**Cost here:** the required check is red on a run green in every job, on
a PR whose author cannot re-run it (`rerun-failed-jobs` → `403 Resource
not accessible by integration`), so the false red is not self-clearing
from the lane that hits it. That is the sharper version of the cost the
WIRE entry recorded: there, a re-run was available.

Added by the CENSUS orchestrator per `docs/prompts/implementer-discipline.md`
§6 — evidence onto the row that already covers it rather than a second
file. Nothing in CENSUS PR 2634's diff touches `ci.yml` or
`scripts/check-run-jobs.py`.
