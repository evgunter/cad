---
id: the-interval-lanes-six-jobs-are-renamed-by-the-workflow-lift
kind: issue
title: The interval lane's six test jobs are prefixed since the workflow lift, so implementer-discipline's twelve-job count reads as a narrowed run
status: open
opened: 2026-09-21
priority: P3
cost: E
needs_ev: true
---



## Finding

`docs/prompts/implementer-discipline.md` §2 tells every implementer
lane that a code-tier run shows **twelve `test (…)` jobs**, and that
*"if you cannot see twelve test jobs and five `k-lint (gate, …)` jobs
on a code-tier run, something narrowed it and you should find out
what."*

Since the interval lane was lifted into its own workflow file
(`.github/workflows/ci.yml:3581` — `uses:
./.github/workflows/interval.yml`), GitHub names that half of the
matrix under the caller's job key: six jobs reading
`interval / test (interval, eps = …, n/2)`, beside six flat
`test (eps = …, n/2)`. The five `k-lint (gate, …)` are unprefixed.
Three of the conditional skips moved under the same prefix.

**So a lane that counts the way the discipline describes finds six and
concludes the matrix was narrowed** — and the doc it is obeying tells
it, in the same sentence, that six-where-twelve-is-expected is the
signature of a break to go looking for. The failure is a lane spending
a round trip proving CI is not broken.

## Measured

Found by CHROME's `chrome/datum-honesty` lane on run 35626720150,
which counted six, went looking for the narrowing, and found the
prefix instead. The same PR's FIRST run, before it merged `origin/main`,
had all twelve flat — so the rename landed on `main` between the two
runs of one branch, which is the cheapest possible way for a lane to
notice and is not a way anyone should rely on.

## What the fix is, and why it needs Ev

The fix is three sentences in `docs/prompts/implementer-discipline.md`
saying the prefix exists and pointing at the `change filter` log
(`LANE`, `EPS`, `KLINT_ROW`) as the thing that actually settles
narrowed-or-not without counting job names at all.

`docs/prompts/` is **Ev's by the merge rules in CLAUDE.md** — it is
the standing discipline handed to every lane by path, and it binds the
orchestrator's own judgement too. So the amendment rides an `[ev]` PR
rather than landing with the unit that found it. `needs_ev: true` here
is that question.

## The general shape, which outlives this instance

A job-NAME is not a stable address: a workflow refactor renames every
job it moves without touching a step. Prose that instructs a reader to
count job names is coupled to a layout nothing gates. The durable
instrument is the run's jobs API and the `change filter` log, both of
which `memories/agent-lane-operations.md` already names — *"the run
record is the instrument; the workflow source is not"* — and neither
of which this count uses.

## Home

CIW: `.github/workflows/*` is this program's territory, and the lift
that caused it is CIW's ground. The amendment itself is in `docs/`
and is Ev's to ratify.

Signed (CHROME orchestrator).
