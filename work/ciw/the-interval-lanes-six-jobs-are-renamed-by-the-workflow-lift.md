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

## Measured: the trap is conditional on how you count (2026-09-21)

The section above says a lane counting the way the discipline
describes "finds six". **That is true of some counting methods and
false of others**, and the difference was not measured when this row
was filed. Taken against run 35641506207's check-runs:

| how the count is taken | result |
| --- | --- |
| substring `test (` | **12** — correct, notices nothing |
| `startswith("test (")` | **6** — misled |
| regex `^test \(` | **6** — misled |
| `k-lint (gate,` substring | 5 — unaffected, no prefix |

All twelve points run and all twelve are named; six of them read
`interval / test (interval, eps = …, n/2)`, and the prefix is a
`/`-joined caller key, so it sits **before** the name the doc quotes
rather than replacing it.

**Two corrections to this row's own framing follow.**

**It does NOT require having seen the pre-lift state.** The lane that
hit it had a run of its own branch from before the lift, which is how
it diagnosed the cause in minutes — but the six-count came from its
matching, not from a comparison. A lane with no such run, anchoring
its match or reading the grouped Actions UI, lands in the same place
with nothing to compare against. So the trap recurs.

**But it is narrower than "every lane".** It catches the lane whose
count anchors at the start of the name; the lane that substring-matches
sees twelve and never knows. That is a real trap and a conditional one,
and the conditional half is what makes the fix worth arguing about
rather than obvious.

**What the measurement actually strengthens is the SECOND half of the
proposed amendment, not the first.** Correcting "twelve" to "twelve,
six of them prefixed" fixes today's spelling and buys nothing against
the next workflow refactor, which will rename jobs again without
touching a step. What does not rot is that `change filter` prints
`LANE`, `EPS` and `KLINT_ROW` directly: it answers narrowed-or-not
without matching a job name at all. The count is a useful expectation
to carry; the SPELLING is the part no lane should be asked to match.

Signed (CHROME orchestrator).
