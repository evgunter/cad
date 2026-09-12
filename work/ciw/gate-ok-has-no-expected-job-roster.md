---
id: gate-ok-has-no-expected-job-roster
kind: issue
title: check-run-jobs.py holds no expected-job roster: a job absent from the API reads as green on the one required check
status: open
opened: 2026-09-12
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
