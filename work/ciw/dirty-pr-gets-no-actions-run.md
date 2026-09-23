---
id: dirty-pr-gets-no-actions-run
kind: issue
title: A PR that goes mergeable_state dirty against a moved main gets NO Actions run on its next push — an absence, not a red — and the lane cannot tell it from a queue
status: open
opened: 2026-09-05
refs: [1910]
priority: P4
cost: E
---


(SEAT orchestrator) Process finding from SEAT-7's fix pass (PR 1910),
filed per the durable-home rule; unowned — CI wiring is CIW's ground.

**Measured.** The fix-pass push `6679e034` (04:41Z, 2026-09-05) got only
the `cursor` and `claude` check suites and NO Actions check suite and no
workflow run, while other branches' runs started normally on either
side of it. Cause: the PR had gone `mergeable_state: dirty` against a
moved `main`; with no computable `refs/pull/N/merge`, GitHub dispatches
no `pull_request` run. The symptom is an ABSENCE: nothing is red, the
required `gate ok` check simply never appears, and a lane polling for
a run sees a queue that never drains. Merging `main` into the branch
restored the run. An empty commit pushed to re-trigger did nothing
(and is the shape the drive-to-green rules forbid anyway).

**What a fix needs.** Either the poll scripts/lane briefs learn the
signal (a push with no Actions suite after ~2 min + `mergeable_state:
dirty` ⇒ merge main first), or a lightweight workflow on
`pull_request_target`/`push` posts a visible "no merge ref — merge the
base" status so the absence becomes a red. Cost of leaving it: ~40
minutes per occurrence, silently.

## Re-homed (2026-09-06)

Moved from `work/issues/` to `work/ciw/` in the tracker-wide cut of 2026-09-06 (Ev's direction, in-chat), which read every open `work/issues/` file and every open code-quality row against every live program's `paths` and opened four programs for the ground none covered. Id, body and header are unchanged except as noted; the directory is the claim (`work/README.md`). CI wiring and the lane poll scripts are CIW's ground, as the body says.

## Merged with `no-ci-run-on-a-conflicting-pr` (2026-09-06, CIW orchestrator)

The same finding reached this slate twice on 2026-09-05, from two
orchestrators who could not see each other: SEAT's (this file, from SEAT-7's
fix pass, PR 1910) and PROPS' (`no-ci-run-on-a-conflicting-pr`, from the
riders lane). Same mechanism, same symptom, same two candidate fixes. This
file survives because it carries the measurement; the other is closed
pointing here, and its two instances are folded in:

- `eba54d6c6` on PR #1977 and `ae5e5c114` on PR #1980 — two fix-pass heads in
  a row with no workflow run. `main` had moved under them in
  `work/props/log.md` and `docs/DOC-LEDGER.md`, tail-append conflicts only,
  which is enough to make a PR conflicting and therefore runless. Merging
  `origin/main` produced a run within seconds, both times.

So the population is **three measured occurrences in one day**, across three
programs, none of them a code conflict — all three were tail-append conflicts
in a log or a ledger, which is the shape this repository generates constantly
by construction.

PROPS' ask (1) is kept and is the cheaper half of the fix above: one line in
`docs/prompts/implementer-discipline.md`'s verification section saying a push
with no run is a conflict to merge out, not a queue to wait on.

## Fourth occurrence, and the first whose base is not `main` (2026-09-15, SUITE/S392)

The three occurrences above share a shape this one breaks: each was a PR
based on `main`, made dirty by a tail-append conflict when `main` moved.
**A stacked PR needs no conflict at all to reach the same state** — its
base is another lane's branch, and that branch advances whenever its own
lane pushes. `#2650` (`suite/s392`, based on `suite/s52`) went dirty at
`03e8733ba` because `suite/s52` moved from `666e34242` to `0621a31b7`
under it. Merging `origin/suite/s52` forward produced a run within two
minutes, exactly as merging `main` did the other three times.

This matters for scheduling rather than for wiring: under merge-only
rules a base branch advancing is not an accident, it is the normal life
of a lane, so **every stacked PR is expected to go runless at least once
per push its base makes**. The three-in-one-day population above is
therefore a floor for how often this fires, not a measure of it, and the
frequency rises with every stacked unit an orchestrator dispatches.

**The discriminator, stated the way this slate's sibling row states its
own.** `red-run-whose-jobs-never-started-reads-as-a-broken-tree` (filed
the same day, by SUITE/D114) is the other half of one family — *the CI
surface reporting something a reading agent will take as a fact about
the tree*. There the misreading is a red that means "no runner"; here it
is a silence that means "no merge ref". Both are settled by one cheap
read before the expensive one:

- **zero steps on a red job ⇒ the job never ran** (that row's tell);
- **no run at all ⇒ read `mergeable_state` before you conclude
  anything about CI**:

```sh
curl -s -H "Authorization: Bearer $GITHUB_TOKEN" \
  "https://api.github.com/repos/<owner>/<repo>/pulls/<n>" \
  | python3 -c 'import json,sys; print(json.load(sys.stdin)["mergeable_state"])'
```

`dirty` there means no `refs/pull/N/merge` exists, so **no
`pull_request` run was ever going to be created**. That is a merge-state
fact, not a CI outage, not a queue and not a narrowed matrix — and it is
invisible from the runs API, which is the only surface a polling lane
looks at. Neither row's tell subsumes the other: a dirty PR has no run
to inspect the steps of, and a never-acquired job's PR is perfectly
clean.

**Cost, measured.** ~45 minutes of one lane's turn, which is consistent
with the ~40 the row already records. It went on: polling for a run at
the new head; polling repo-wide to establish that runs were being
created for other branches (they were, which *excluded* the outage
hypothesis and left no other); closing and reopening the PR to force a
`reopened` event, which is in the workflow's `types` list and still
produced nothing; and re-reading `ci.yml`'s `on:` block for a branch
filter that does not exist. Every one of those is a reasonable next step
from the runs API alone, and the one cheap read that would have ended it
at the first minute is not reachable from there.

**Whose cost it was.** SUITE's orchestrator stacked this unit on
`suite/s52` deliberately, so the lane could start before `#2639` merged,
and the dispatch did not say that the base advancing would silently stop
the lane's runs — because the orchestrator did not know it either. The
trade was probably still right: the unit needed S52's shared home and
waiting would have idled it. But the row should carry it, so the next
orchestrator can weigh a stack against a known cost rather than an
unknown one, and so that the fix below is understood to be worth more
than three occurrences suggest.

**What this adds to the fix, not a new ask.** The cheap half the plan
already schedules — one line in
`docs/prompts/implementer-discipline.md`'s verification section — should
say the base, not `main`: *a push with no run is a conflict with the
PR's own base to merge out, not a queue to wait on.* A lane told to
merge `main` out, on a PR based on a lane branch, would merge the wrong
thing and stay runless.

## Fifth occurrence, and the first measured RATE (2026-09-15, S-TINT orchestrator)

The row says this shape is one "this repository generates constantly by
construction" but carries no frequency. `#2690` (`tint/orchestrator`,
based on `main`) supplies one, because it stayed open across a busy
stretch and its base merges were timed:

**Four base merges in ~100 minutes; three of them conflicted, and all
three conflicts were `docs/DOC-LEDGER.md` alone.** Between the third and
fourth, `origin/main` advanced **22 commits in about 30 minutes**. The
occurrence proper is the fourth: the push at `39dbf40eb` produced no
Actions run for twenty minutes while `mergeable_state` read `dirty`,
and merging `origin/main` out produced a queued run within seconds.

So the rate is not "three in a day" but roughly **once per base merge
for any PR open longer than main's inter-merge interval**, and that
interval is currently well under the ~30 minutes one CI cycle takes.
A PR that needs two cycles is more likely than not to go runless at
least once, without anyone doing anything wrong.

**`docs/DOC-LEDGER.md` is why, and it is not incidental.** Every program
appends a per-merge-deletion entry to that one file at the end of every
unit, so the file takes a tail-append from every lane in the repository
and collides with every other open branch that has also appended. It is
the single highest-collision file in the tree by construction, and
nothing about the ledger's purpose requires one file — this row does not
ask for that change, but a fix that only teaches lanes to recognise the
silence leaves the collision rate untouched.

**Who was fooled this time: the orchestrator, not a lane**, and it
reported "queued or just starting" to its user before reading
`mergeable_state`. The row's discriminator is right and cheap; what this
adds is that the seat dispatching the work is no better placed to guess
than the seat doing it, so the cheap half belongs in a place both read.
`docs/prompts/implementer-discipline.md` is a lane's file; an
orchestrator reads it too, which is the argument for putting it there
rather than in a lane brief.

## A fourth instance, and the cost measured (2026-09-16, INSTR)

INSTR unit 0's fix pass hit this exactly as the title describes. The
push `35d834490` on PR 2735 produced **zero** workflow runs — no queued
job, no failing check, no error anywhere. The lane read it as a queue
delay and **polled for roughly twenty minutes** before reading the PR
object and finding `mergeable_state: dirty`. Merging `origin/main` in
produced a full 39-job run on the next push.

Two details worth adding to this row's evidence rather than to a new
one:

- **The mechanism is visible in the workflow's own prose.** `ci.yml`
  around its checkout argues that the merge ref's first parent *"is by
  construction the exact base state the merge was built against"*,
  because the event payload's `base.sha` can be newer. So the diff
  classification every gated job depends on is built from
  `refs/pull/N/merge` — which is precisely the ref GitHub cannot
  compute for a dirty PR. The silence is not incidental to this
  repository's design; it follows from it.
- **`pull_request_read get_status` cannot see this either.** It returns
  `pending / total_count 0` for every PR in this repo, dirty or not,
  because the repo reports through check runs rather than the legacy
  commit-status API. A lane that polls `get_status` for "is CI done"
  gets the same answer on a healthy PR and on an ungated one. The
  workflow run is the record.

**A second open row on this slate names the same defect.**
`work/ciw/an-unmergeable-pr-is-silently-ungated-not-visibly-red` was
opened on 2026-09-16 — *"A PR whose merge ref cannot be computed gets
ZERO check runs, which reads as green unless you count jobs"*. That is
this row's subject from the reader's side rather than the lane's, and
`no-ci-run-on-a-conflicting-pr` already closed into this one on
2026-09-06. Three rows for one defect is the duplicate cost
`work/README.md` names, and **which of the two open rows survives is
CIW's call, not INSTR's** — recorded here rather than acted on, because
merging another program's rows across the fence is not a passing
lane's to do. This evidence is filed on this row because it is the
older and the one the closed row already points at.
