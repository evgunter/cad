---
id: merge-queue-trial
kind: unit
title: Merge queue trial: designed and costed, then found unavailable — GitHub offers merge queues only to organization-owned repositories
status: closed
opened: 2026-09-04
refs: [f3-recosting-on-a-public-repo, merge-order-semantic-break-reaches-main, klint-row-still-sampled, reinstate-full-configuration-runs, render-lanes-red-at-missing-merge-ref, 1796, 1823]
pr: 1887
branch: ciw/merge-queue-unavailable
closed: 2026-09-05
---

# CLOSED: MERGE QUEUE IS NOT AVAILABLE HERE AND WILL NOT BE

**Read this before anything below it.** Everything under it is the
design for a merge queue, and **there will be no queue.** This unit is
closed as a closed avenue, not parked and not deferred: nothing is
waiting, and there is nothing left to decide.

**The feature is not offered to this repository.** Ev went to turn the
queue on after PR 1845 landed and found no toggle. The cause is not a
misconfiguration, not a missing branch-protection rule and not a plan
tier. The grant, quoted in full from `github/docs@main`,
`data/reusables/gated-features/merge-queue.md` (read 2026-09-05):

> Pull request merge queues are available in any public repository owned
> by an organization, or in private repositories owned by organizations
> using {% data variables.product.prodname_ghe_cloud %}.

`evgunter/cad` is a **personal-account** repository: the GitHub API on
`repos/evgunter/cad` (repository id 1302372371) reports `"visibility":
"public"` and, on its owner, `"type": "User"`. Neither arm of that grant
reaches it — the first needs an organization owner, the second needs an
organization owner *and* GitHub Enterprise Cloud. There is no setting
that produces the toggle, so there was never anything to troubleshoot.

**And the one thing that could have changed that is answered.** Ev,
2026-09-05: *"ok i don't plan to move this to an organization"*. So the
ownership gate is permanent for this repository, the queue is
unreachable for as long as that holds, and this item closes on that
ruling rather than waiting on it.

**Going public did not and could not change the outcome.** The
repository went public on 2026-09-03, and that removed the *billing*
gate on Actions minutes — which is what made every re-costing in this
program possible. Merge queue is gated on **ownership**, not on
visibility and not on billing. Two different gates, and this unit
conflated them.

**A trap that is NOT what happened here, recorded because it is real and
costs an hour to chase.** The same documentation
(`content/repositories/configuring-branches-and-merges-in-your-repository/configuring-pull-request-merges/managing-a-merge-queue.md`)
says: *"A merge queue cannot be enabled with branch protection rules
that use wildcard characters (`*`) in the branch name pattern."* Ev
created a branch protection rule and the toggle still did not appear,
which is the evidence that the wildcard clause is not the cause. If some
future repository shows this shape, check the owner type first.

## WHAT SURVIVES THIS, AND WHAT MUST NOT BE RIPPED OUT

* **`gate ok` stays. It is not merge-queue residue.** It runs on every
  `pull_request` run today, it reads the run's own job list, and its
  reader `scripts/check-run-jobs.py` is selftested in `discipline` and in
  `local-scripts/ci-local.sh`. Nothing about it needs a queue.
  **Requiring it as a status check also remains available** — branch
  protection with required status checks works on a public
  personal-account repository, and that is a different feature from a
  merge queue. The argument below for *one computed-proof name* is
  therefore still a live argument about required checks; only its
  merge-queue framing is dead. `scripts/check-run-jobs.py` and the
  `page_is_whole` paging guard added to `scripts/opt-level-calibrate.py`
  in the same PR are likewise unaffected.
* **The `merge_group` wiring in `ci.yml` is now dead, and removing it is
  NOT this item's call.** Two lines of it are executable and can never
  fire on this repository: `.github/workflows/ci.yml:40-41` (`on:
  merge_group` / `types: [checks_requested]`) and the
  `github.event_name != 'merge_group'` conjunct in `renders`' guard at
  `.github/workflows/ci.yml:4592`, which is now always true. Everything
  else is comment prose that describes a mechanism this repository will
  never run: `ci.yml:17-38` (the trigger's own block, whose first line
  reads *"INERT UNTIL SOMEONE ENABLES A QUEUE"* — nobody can),
  `ci.yml:187-194`, `ci.yml:871-877`, `ci.yml:1282-1287`, `ci.yml:3903`,
  `ci.yml:4566-4591` (the `renders` exclusion's argument) and
  `ci.yml:4618-4717` (`gate-ok`'s key, which is merge-queue-framed
  around a job that is not). They cost no run, no job and no minute —
  `merge_group` fires only where a queue exists — so leaving them costs
  reading, and removing them is a workflow edit on a file several lanes
  touch. The trade is written up for the orchestrator in this PR's
  description; **nothing was removed here, deliberately.**
* **Two facts below are durable and worth quoting forward**, independent
  of any queue: (a) GitHub builds one merge group **per queued pull
  request**, so merge limits and batch size are not a CI-cost lever —
  which is the mechanism error that PR 1796's 44-job-min/h figure rested
  on, and it should not be quoted from that document again; and (b) the
  2026-09-04 post-un-sampling cost measurements (44.8 job-min and 528 s
  wall for a code-tier run, 5.76 merges/h, 41 % code-tier) are real
  measurements of this repository's CI and are reusable by anything that
  needs them.

## THE PROCESS FAILURE, STATED PLAINLY

This unit costed a merge queue to two decimal places — 99.5 job-minutes
an hour, ρ = 0.39, a latency simulation over 138 observed merge arrivals
— argued the required-check design against three alternatives, shipped a
runbook with nine branch-protection settings and their reasons, and
carried a section titled *WHAT THIS UNIT COULD NOT DEMONSTRATE* with six
honest entries in it. **Not one of them is "whether this repository can
have a merge queue."** Neither the unit, nor its style review, nor the
orchestrator ever asked. The question is one API field and one sentence
of documentation, and it decides everything the unit did.

It is the pattern this program has been filing all day, arriving at the
program itself: **a true mechanism carrying an unchecked premise.** Every
number here is right; the premise under all of them was never read. The
unit was reading GitHub's merge-queue documentation closely enough to
quote the merge-limits page and the skipped-checks page verbatim, and the
availability banner sits on the same set of pages.

**The lesson, in one sentence a future lane can act on: check that the
feature is available to this account and this repository BEFORE costing
it — the entitlement check belongs in front of the measurement, not
behind the design.**

## THE DESIGN AS IT WAS WRITTEN (2026-09-04) — KEPT AS WHAT WAS LEARNED

Everything from here down is the unit as it was argued and measured on
2026-09-04, before anyone checked whether the feature existed for this
repository. **It is kept, in full and unsoftened, as the record of what
was learned** — the measurements are real measurements of this
repository's CI, the batch-size mechanism is a durable fact about
GitHub, and the required-check argument survives into the
branch-protection question that a queue was only one consumer of. **None
of it is an instruction to do anything.** Read it as a report, not as a
plan.

Opened on Ev's ruling of 2026-09-04, taken after reading the costing in
PR 1796: **trial a merge queue.** This unit is the design and the
workflow support. **It does not enable anything** — enabling is a
branch-protection setting that changes how every agent in this
repository merges, and a misconfiguration blocks all merging. The
runbook was written for whoever flips it; nobody ever did, and on
2026-09-05 it was established that nobody can (top of this file).

## The defect the queue is for, in one paragraph

Two defects reached `main` on 2026-09-03/04 by the same mechanism: two
pull requests, each correctly green against a base that lacked the
other, merged minutes apart into a state **no run had ever compiled**.
M10's `node_error_tag` values met LIB's `TAG_INVENTORY` gate 7 minutes
apart; CHROME's exhaustive `blamed_mates` met M10-7's `Unleverable`
variant 22 minutes apart, leaving `main` non-compiling for 34 m 25 s
(`work/ciw/merge-order-semantic-break-reaches-main`). A restored push
gate *detects* this about 6 minutes after landing and, measured against
the two instances, catches 1 of 2 — the other run is cancelled by the
next push. A merge queue tests the prospective merged state **before**
it lands, so `main` never carries it. That is why Ev chose it over the
push gate, at the same runner cost.

## THE CENTRAL FACT, VERIFIED AT THE SITE

PR 1796 recorded that `ci.yml`'s `!= 'push'` spelling means a
`merge_group` event already runs the full gate with no edit. **Checked
here rather than taken, and it holds.** Every job's `if:` was read out
of the parsed workflow, and the population divides three ways:

| jobs | guard | under `merge_group` |
|---|---|---|
| `discipline`, `fmt`, `clippy`, `clippy-all-features`, `build`, `test`, `build-interval`, `lint-interval`, `test-interval`, `interval-backend`, `oracle-certify`, `step-import`, `python-suite`, `k-lint`, `docs-only` | `github.event_name != 'push' && …` | **all run** |
| `filter`, `mirror` | no `if:` at all | run |
| `cache-prime`, `cache-prime-interval` | `== 'push' \|\| == 'workflow_dispatch'` | skipped — they are cache writes, not gates |
| `renders` | `!= 'workflow_dispatch' && …` | **would have run**; excluded here, see below |

So the wiring is the `on:` key and nothing else, and the `!= 'push'`
spelling — argued into the file on 2026-08-28 as *"a trigger added here
runs the gate by default and has to be excluded deliberately, which is
the direction that fails safe"* — is the reason this unit costs a
trigger rather than a sweep.

**Three qualifications PR 1796's enumeration did not carry.** None of
them changes the verdict. All three are written back into that document
as an addendum: `renders` and "degrades silently" as corrections to its
verdicts, and the change filter as a closure of a question it left open.
(It first carried two of them; the `renders` reversal was added on
2026-09-04 after a style review pointed out that this unit overturned
that document's "correct by default" and left the sentence standing
there.)

1. **`renders` would also have run**, because it is excluded from
   `workflow_dispatch` and not from everything but `pull_request`. This
   PR excludes it from `merge_group` too, with the reason at its key: a
   merge group has no branch, so `push_to` is empty and the drift check
   would post against a `gh-readonly-queue/…` sha that is deleted at the
   merge, while the PR run has already rendered this merge preview and
   `main`'s push run re-renders and re-baselines after it. At the
   2026-09-04 medians that is **330 job-seconds of the 2686 a code-tier
   run costs** (median over the same n = 9), about an eighth of what the
   queue adds.

   **What IS given up is on the other side, and this PR is what creates
   it.** "Nothing is given up: a render lane has never blocked a merge"
   was true of the merge_group side and false of the pull-request side of
   the same change: `gate-ok` carries `- renders` in its `needs:`, so once
   `gate ok` is a required check a FAILED render lane blocks the pull
   request — the first time a render lane can. That is decided, kept and
   argued in *the required-check list* below, on a measured rate.
2. **The change filter's basis is correct under `merge_group` with no
   edit**, which PR 1796 left as "the open question". GitHub builds one
   merge group **per queued pull request** — group N contains the base
   branch plus every PR ahead of it plus that PR — so group N's parent is
   group N-1, and `filter`'s `git rev-parse HEAD^1` (ci.yml, the
   `classify the change set` step) yields exactly the one PR that group
   adds. The tree its parent adds was gated by the group before it. The
   `filter` job checks out at `fetch-depth: 0`, so `HEAD^1` resolves.
   *This holds only while "Only merge non-failing pull requests" is
   enabled* — see the settings below, where that is the reason for it.
3. **The two test-cost report steps do NOT degrade silently.** PR 1796
   read them as "they degrade silently, which is this repo's least-liked
   failure shape". `BASE_SHA` is empty under `merge_group` and
   `scripts/base-test-listing.sh:86-88` answers with a stated skip —
   *"this run is not a pull_request run, so it has no base tree to diff
   against"* — printed into the job log and the step summary. The
   `work tracker territory (advisory)` step is already
   `if: github.event_name == 'pull_request'` and simply does not run.

## THE MEASUREMENT, RE-DERIVED (2026-09-04, this unit)

Every figure in PR 1796 predates the un-sampling that landed at
`73f1e83d`, **2026-09-04 10:02:26Z** (PR 1823), which is exactly the
change that made a required-check list possible. So the numbers are
re-taken, not quoted, and they moved a lot.

**Populations, stated so they reproduce.** (a) `pull_request` runs of
`ci.yml` created after `2026-09-04T10:10:00Z` with `conclusion` ∈
{success, failure}: **22 runs, 9 code-tier** (a `build + archive` job is
live). All 9 carry **12** `test (…)` jobs, which is the un-sampled matrix.
(b) `push` runs on `main` created `2026-09-03T15:47Z … 2026-09-04T15:45Z`
= **23.96 h, 138 runs**, cancelled included — a cancelled push run still
classifies, and excluding them would bias the tier mix.

| figure | value | n |
|---|---|---|
| code-tier run, **job-seconds** | **2686 s = 44.8 job-minutes** median (p25 1654, p75 3143, max 3609) | 9 |
| code-tier run, wall (created → last job end) | **528 s** median (p25 423, p75 567, max 1070) | 9 |
| live jobs per code-tier run | **27** median (26–31) | 9 |
| docs-tier run, job-seconds / wall | 68 s / 42 s median | 13 |
| merges (pushes to `main`) | **5.76/h** | 138 / 23.96 h |
| of them code-tier (a render lane ran) | **41 % → 2.38 code-tier merges/h** | 57 |
| merge inter-arrival | mean **630 s**, median 243, p25 111, p75 575, p90 1295, max 5942 | 137 |
| queue delay today (run created → first job start) | median **3 s**, p90 4–22, p99 ~90, max 305 | 159 |

PR 1796's 24.4 job-minutes per code-tier run has become **44.8**, and
its 1.97 code-tier merges/h has become **2.38** on a busier window. The
first is the un-sampling; the second is the window.

**What a queue costs in runner time.** One full gate per enqueued pull
request — **and a queue run is not a PR run.** The 44.8 job-minutes above
is a `pull_request` run and it INCLUDES the render lanes, which this PR
excludes from `merge_group`. Priced on what a merge group actually runs,
per-run and then taken to the median rather than by subtracting one
median from another (n = 9 code-tier runs, 10:10Z–15:45Z):

| figure | value |
|---|---|
| code-tier `pull_request` run | 2686 s = **44.8 job-min** median |
| its render lanes | 330 s = 5.5 job-min median |
| **code-tier MERGE GROUP run** | **2411 s = 40.2 job-min** median (p25 1314 s, p75 2846 s, max 3457 s) |

> 2.38/h × 40.2 job-min + 3.38/h × 1.13 job-min = **99.5 job-minutes an
> hour = +1.66 mean concurrent jobs**.

**That corrects a figure this unit published at 110 job-min/h**, which
priced 2.38 render lanes an hour a merge group will never run — about
13 job-min/h of it. The correction is conservative and moves no
recommendation. (99.5 rather than the ~97 a flat subtraction gives:
medians do not subtract, and the per-run difference is the quantity the
queue pays.)

**The service time does NOT need the same correction, and that was
measured rather than assumed.** The 528 s wall is also a PR-run wall
that included the render lanes — but recomputing each of the 9 runs'
wall clock with the render-lane jobs dropped moves the median **not at
all** (528 s → 528 s; one run of the nine moves at all, 369 → 350 s).
The render lanes are not on the critical path, so the latency table
below and ρ = 0.39 stand unchanged.

For scale, the same post-un-sampling window's `pull_request` runs held
**71 job-min/h** — but that window is quiet (4 runs/h against the 10.3/h
PR 1796 measured over 14.45 h), so **that ratio is not a like-for-like
comparison and must not be read as "the queue doubles CI"**. What is
like-for-like: today's queue delay is 3 s at the median and there is no
backlog for +1.66 to join.

**PR 1796's queue figures do not carry forward, and one of them was
wrong in mechanism, not only in magnitude.** It priced "merge queue,
batch ≤5" at **44 job-min/h against the push gate's 48**, from a
simulation in which batching reduced the number of CI runs. GitHub's
merge queue does not work that way — see the next section — so the
queue's cost is one gate per PR at any batch size, and at today's job
set that is **99 job-min/h**, not 44. The queue's *advantage* over the
push gate is unchanged and is not about cost: it prevents rather than
detects, and it needs no concurrency design pass.

## BATCH SIZE, AND WHY IT IS NOT THE LEVER THE QUESTION ASSUMES

GitHub's own documentation settles this, and it contradicts the model
PR 1796 simulated. Two separate settings are called "batch size" in
conversation and neither reduces CI:

* **Merge limits** (minimum / maximum pull requests to merge, and a wait
  time) — *"Merge limits do not combine `merge_group` **builds**. Merge
  limits only affect merges to the base branch once one or more
  `merge_group` has satisfied build checks."*
* **Build concurrency** — *"The maximum number of `merge_group` webhooks
  to dispatch (between 1 and 100), throttling the total amount of
  concurrent CI builds."*

Because a merge group is created **per queued pull request** (group N =
base + PRs ahead + PR N), **every enqueued PR costs one full gate run
whatever the merge limit is.** Raising the maximum-to-merge does not
merge five PRs on one run; it merges five *already-built* groups in one
push to `main`. So:

**The runner cost of the queue is fixed at ~99 job-min/h and the batch
size cannot move it.** What the levers do move is latency and waste.

### Latency, simulated on the 138 observed merge arrivals

Service times are this unit's measured medians (528 s code-tier, 42 s
docs-tier); the server model is GitHub's: group N's build starts when a
build-concurrency slot frees, and PR N lands when its own group and every
group ahead of it have passed.

| build concurrency | code-tier PR: median | p90 | max | docs-tier: median |
|---|---|---|---|---|
| **1** | **1312 s (21.9 min)** | 2861 s | 3774 s (63 min) | 381 s |
| 2 | 528 s | 664 s | 912 s | 71 s |
| 3 | 528 s | 528 s | 668 s | 42 s |
| **4 and above** | **528 s** | **528 s** | **528 s** | 42 s |

The maximum number of merge groups in flight at once, over the same
24 hours, is **4**. That is why the table flattens at 4: at that width
every PR's latency is exactly one run's wall clock and nothing queues.

**Recommendation: build concurrency 5** — the observed peak plus one.
Concurrency 1 is the setting that produces a backlog; the table is not
close about it (1312 s median, and a 63-minute worst case at a
utilisation of only ρ ≈ 0.39, because arrivals are bursty at p25 = 111 s).

**Recommendation: maximum pull requests to merge = 1, minimum = 1.**
Maximum 1 keeps one merge ↔ one push to `main`, which is what every
existing mechanism assumes — the render re-baseline commit, the
`STATUS.md` regeneration, and every reading in this program that counts
push runs per merge. It costs nothing: merging is an API operation, not
a build. Minimum 1 means nothing ever waits for a group to fill, so the
wait-time setting never fires. Raising the maximum later is a pure
throughput knob with no effect on CI cost, and the reason to raise it
would be push-run churn on `main`, not merge latency.

### What a failed group costs, and what a reader sees

**There is no bisect.** Because groups are speculative and each differs
from its parent by exactly one PR, a failing group names its PR
directly. GitHub's documented behaviour: *"When the GitHub API receives a
failing status for `main/pr-1`, the merge queue automatically removes
pull request #1 from the merge queue"*, then *"recreates the temporary
branch with the prefix of `main/pr-2` to only contain changes from the
target branch and pull request #2."*

So the cost of one bad PR is: it is ejected, and **every group behind it
is rebuilt** — at the observed queue depth that is 0 to 3 rebuilds,
528 s and 44.8 job-minutes each. At the observed rate of composition
defects (2 in 45.66 h) that path is rare, but "rare" here is n = 2, and
the rate that actually drives it is the FLAKE rate on a merge group,
which nothing in this repository has measured. Under a queue a flaky
test stops being a red someone re-runs and becomes an ejection from the
queue plus a rebuild of everyone behind.

**A RECREATED GROUP IS A SECOND RUN, AND NOTHING CANCELS THE FIRST.**
`ci.yml`'s run-level `concurrency:` group is per-ref, and a merge-group
ref is distinct per group — which is why the queue needs no concurrency
design pass, and which is exactly why this happens: when a group is
recreated after an ejection, or rebuilt because a pull request *"jumping
to the top of a merge queue will cause a full rebuild of all in-progress
pull requests"*, the new group gets a **new ref**, hence a **new
concurrency group**, so the superseded run is not cancelled by this key
and runs to completion on the runners. On a pull request a superseded run
IS cancelled; in a queue it is not.

**Both figures in this unit are silent about that, and neither models
it.** The cost figure prices one gate per enqueued pull request and no
rebuilds; the latency table says in as many words that no failures are
modelled. So the cost of an ejection is: 0–3 rebuilt groups at 40.2
job-min each, **plus** whatever the superseded runs burn before they end
on their own — and none of the three is in the 99 job-min/h. This is
stated as a named limit of both numbers rather than modelled, because the
quantity that drives it is the merge-group failure rate, which nothing
here could measure. It is the second reason the runbook asks for a flake
rate if the trial is kept.

**What a reader sees, and it is the weak spot.** The failing run is a
`merge_group` run on `refs/heads/gh-readonly-queue/main/pr-N-<sha>`. It
is **not** on the pull request's own checks tab — the PR shows a
"removed from the merge queue" event, and the run is reached from the
Actions tab or the queue view. That is worse than today's reading
experience for a red, and it is the one thing the trial should be judged
on besides latency.

## THE REQUIRED-CHECK LIST: ONE NAME

**Require exactly one check: `gate ok`.** This PR adds the job that
reports it.

**`docs-only ok` already claimed part of this role, and the two are now
declared a pair rather than left to overlap.** That job's key read *"a
marker job with no check in it — it echoes the tier so a docs-only PR has
a green **required** status"*, written for the same imagined branch
protection ~3,700 lines above. It is not the required status and is not
meant to be: `docs-only` asserts WHAT THIS RUN CLASSIFIED AS and nothing
about anybody's conclusion — which is why it is the marker a DRAFT run
keeps, when `filter` has zeroed every `RUN_*` flag — while `gate ok`
asserts every other job's conclusion, which `docs-only` never could. Both
keys now say so. The tier marker has three spellings in `ci.yml` (this
job, `filter`'s `the configuration this run gates` and draft-verdict
steps, and `gate ok`'s summary line); a fourth is a reason to collapse
some of them rather than to add it.

A required status check is required *by name*, and naming this
workflow's gating jobs one by one fails here for **two** reasons, both of
which a name list cannot fix:

1. **Two of the names are computed.** `test (eps = …, …/2)` and
   `test (interval, eps = …, …/2)` interpolate `eps_rows`. A dispatch
   input cannot reach a merge_group run (a queue takes no inputs) — but a
   required-check list is **shared with pull requests**, where a
   dispatch's narrowed names show up. A gate whose names a run's inputs
   can shrink is a gate those inputs can shrink. (A commit trailer was
   the second spelling here until 2026-09-04, when it was deleted.)
2. **A list goes stale silently.** A job added to `ci.yml` and forgotten
   in a branch-protection setting is a gate nobody removed and nobody
   runs.

**A THIRD REASON WAS LED WITH UNTIL 2026-09-04 AND IS WITHDRAWN.** It
read: most jobs here report `skipped` on a docs-tier merge, and whether a
skipped check satisfies a required check is not something this unit
established — so one name removes the question rather than betting on the
answer. **The question was already answered, in GitHub's documentation,
and reading was all it took.**
`content/pull-requests/how-tos/merge-and-close-pull-requests/troubleshooting-required-status-checks.md`
(`github/docs`, `main`, read 2026-09-04), under *Handling skipped but
required checks*, is a three-row table whose second row is:

> | A job is skipped by a conditional | The job reports "Success" | |

and one section above it:

> Successful check statuses are `success`, `skipped`, and `neutral`.

The row that *does* leave a check pending and block merging is the
**workflow-level** skip — path filtering, branch filtering, a `[skip ci]`
commit message — and no `if: run_build` job here is that shape. So the
failure the reason feared (a queue that never merges a documentation
change) is documented not to occur. The design does not change; the
reason does, and the two above are what carry it. The same page also
*earns* this job its `always()`, which is a citation this design deserves
rather than a correction:

> | A job depends on a failed job | The dependent job is skipped and may
> not block merging | **Use `always()` with `needs` for required checks
> that depend on other jobs.** |

`gate ok` runs on every event but `push`, with `if: always()` and
`needs:` naming all twenty other jobs, and asserts two things that fail
closed:

* **every job of the run except itself reached a terminal state** — a job
  still running is a job missing from `needs:`, and it is a red naming
  that job, not a pass. This is what keeps point 2 from happening: the
  list cannot go stale silently, because the check reads the run rather
  than trusting its own `needs:`;
* **every one of them concluded `success`, `skipped` or `neutral`** —
  those three because they are **GitHub's own definition of a passing
  check** (the sentence quoted above), not because of anything this
  workflow emits. `failure`, `cancelled`, `timed_out` and
  `action_required` are reds.

**The `neutral` entry's old reason was wrong and the tree said so.** It
read "`neutral` passes because that is how a render lane reports drift it
re-baselined". A render lane's drift signal is a **check run**, posted
through `POST /check-runs`, precisely because — in
`.github/actions/rebaseline-lane/action.yml`'s own words — *"a workflow
JOB cannot conclude `neutral`"*. This job reads
`/actions/runs/{id}/jobs`, so it can never see one: run 33894380300's
listing shows all three `render lanes / …` rows as `success`. The entry
stays, because the accept-list is GitHub's definition of a pass and not
an enumeration of what this workflow happens to emit. **The second-order
fact that reason was hiding is worth stating on its own: `gate ok`
summarises JOBS AND NOT CHECK RUNS**, so the `render drift (…)` and
`render re-baselined (…)` neutral checks are outside the one name
entirely. That is intended and was nowhere written down.

An unreadable jobs API, a paged job list, or a run in which no job but
`gate ok` exists are each a red with the reason printed.

**THE GREEN LINE SAYS WHAT IT GATED.** Once `gate ok` is the only
required check, its output is the whole reading surface for *what did
this run gate?*, and a count answers a different question — this
repository elsewhere calls a green job name over a silent skip "the
failure mode to avoid". So the summary prints the tier and `run_build`
beside the tally, and on a run whose `RUN_*` flags were zeroed (a
docs tier, a draft) it says in a sentence that every build row was
skipped and that the green is about the rows that ran.

**THE READER IS A SCRIPT, NOT A HEREDOC** (`scripts/check-run-jobs.py`).
It went in as 56 lines of inline python and came back out on the same
day, for the reason every other python in `ci.yml` is a `scripts/*.py`
invocation with a `--selftest`: on a real run exactly ONE of its seven
decision paths executes — the all-green one — and the other six are the
ones that decide whether a red run can report green. Inline, nothing in
the tree could re-drive them. Out here `--selftest` drives all seven
against a stub `gh` (ten cases: green, green-with-nothing-built, a failed
job, a job still running, a paged job list, an empty population,
`neutral`, `gh` failing, a zero exit with a body that is not the listing,
and a missing `$GITHUB_RUN_ID`), and it runs in `discipline` and in
`local-scripts/ci-local.sh`'s `discipline()` — the
`scripts/base-test-listing.sh` argument, arriving at the one job that is
the entire required check.

**The consequence to state plainly: requiring `gate ok` gates pull
requests too.** GitHub couples them — *"Merge queue and pull requests
checks are coupled and configured under branch protection rules or
rulesets"* — so there is one required-check list and it applies to both.
Today `CLAUDE.md` says agents merge their own PRs; after this they
cannot merge one whose gate is red. That is a tightening, it is
probably wanted, and it is not something a lane should slip in without
saying so.

**No render check may be in the list.** `renders` does not run under
`merge_group` (this PR's exclusion), so a render check name would never
report and the queue would stall until the status-check timeout.

**But `renders` IS in `gate-ok`'s `needs:`, so PR-side render failures
now block. Decided, kept, and here is the reasoning.**

A failed render lane could not block anything before this PR:
`rebaseline-lane`'s header argues neutral-not-red from *"nothing in this
repo is branch-protected and agents merge their own PRs, so a red X was
never actually blocking"*. Requiring `gate ok` ends that, on the
pull-request side only (a merge group runs no render lane at all). Three
things decided it:

1. **The rate is measured, and the class that produced the reds is
   gone.** `work/ciw/render-lanes-red-at-missing-merge-ref` measured
   **103 failed render-lane jobs in 89 runs** — every one of them
   `fatal: couldn't find remote ref refs/pull/N/merge` at checkout. That
   is the **pre-fix** rate: PR 1724 landed as **`a5d9f41a`**
   (2026-09-04T01:31:21Z, an ancestor of `main`) and gave `render.yml` a
   `checkout target` job that resolves the object and hands the lanes a
   SHA. Re-measured over every completed `ci.yml` run since that merge —
   **n = 259 runs carrying render lanes, 777 render-lane jobs, to
   2026-09-04T16:46Z** — there are **8 failed render-lane jobs, and none
   of them failed at checkout.** Per render-bearing run that is 3.1 %;
   per render-lane job, 1.0 %. Over the window immediately before the
   merge the same reading gives 5 failures in 116 render-bearing runs,
   **all five at checkout**.
2. **The eight residual failures are the lane saying something about the
   tree**, which is what a gate is for. One is `demo tour (…)`; seven are
   `the real gallery opens in the viewer (GUI-4 acceptance)` inside 25
   minutes on 2026-09-04, an incident that also reddened `main`'s own
   push run (33841011297). Those seven are one break, and blocking on it
   is the intended behaviour of a merge gate, not collateral — though it
   is also the shape to watch: a break on `main` reddens every PR that
   has merged `main`, and under a required check that is a fleet-wide
   block rather than a fleet-wide red X.
3. **Dropping `renders` from `needs:` would not have removed the
   blocking.** The sweep reads the RUN's job list, not `needs:`, so the
   render lanes stay in the population either way; without the `needs:`
   edge they would simply more often be caught *still running*, which
   claim 1 reds. Actually excluding them would take an exemption inside
   the sweep — and an exemption inside the sweep is the thing the sweep
   exists to prevent.

**If it turns out to bite, the lever is at the lane and not here**: make
a render lane's hard failures `continue-on-error` or `neutral` in
`render.yml`, which is a change to what the lane MEANS, argued at the
lane. Removing one name from a sweep that claims to read the whole run
is not.

## THE K-LINT DEPENDENCY, STATED ACCURATELY

Ev authorised un-sampling `k-lint` in chat on 2026-09-04
(`work/ciw/klint-row-still-sampled`, which also now carries a measured
incident: `#1756 → #1775`, where `k-lint (gate)` reported green with
`demos tour fmt + clippy` **skipped**, because the drawn row did not
carry it). **This unit designs against that answer and does not build
it** — the k-lint change is that item's, and this lane's territory is
`ci.yml`'s triggers, not its k-lint job.

**The obvious version of the problem is false and should not be
repeated.** "A sampled row cannot be a required check" does not hold
here: `k-lint (gate)` is a single job with a fixed `name:` and no
matrix, so its check name is stable whatever row it draws, and under
`gate ok` it is not named in a branch-protection setting at all. The
required-check list works today.

**The real problem is that the queue draws its own row.** The seed is
`SEED: ${{ github.event.pull_request.head.sha || github.sha }}`, which
under `merge_group` is the merge group's head — a commit that did not
exist when the PR was reviewed. So a queue run draws a row
**independently of the one the PR run drew**, and three things follow:

* a pull request green on row X can be **ejected from the queue** by
  row Y, for a defect its own branch never displayed;
* the author cannot reproduce it by re-running their PR, because a
  re-run of the same head SHA draws the same row — it takes a new commit
  and a way to name the row, which after 2026-09-04 is a
  `workflow_dispatch` and nothing else;
* re-queueing draws **again**, so the ejection may not reproduce in the
  queue either: a failure that looks flaky and is perfectly
  deterministic.

That is the ordering dependency, and it is the one thing in this design
that must land before the switch is flipped:

> **Un-sample `k-lint` first (`work/ciw/klint-row-still-sampled`), then
> enable the queue.** With it done, every dimension of the job set is
> fixed and a queue run gates the same configuration the PR run did.

Enabling before it is not a hole in the gate — a drawn row still gates
something — but it makes queue ejections unreproducible, which is the
worst property a merge gate can have on the day it is introduced.

## WHAT THE QUEUE DOES NOT FIX

Placed here, and repeated at the top of the runbook, because the
temptation after a queue lands is to read every `main` red as impossible.

1. **A defect that needs a configuration nobody runs.** The queue runs
   the same job set as a PR run. `k-lint`'s remaining 1-in-5 draw is one
   (above). A configuration with **no row at all** is another, and this
   repository has open instances of it:
   `work/lib/the-python-feature-half-of-pncad-py-is-linted-by-no-ci-row`
   and `work/issues/gui-wasm-build-is-not-gated-at-all`. A queue gates
   what CI runs; it does not discover what CI does not run.
2. **Tier scope.** A merge group classifies by tier like any other run,
   so rows a PR's tier skips are skipped in the queue too. That is the
   class of the closed `main-latently-red-at-tier-all`, and a queue does
   not address it.
3. **Anything after the merge.** A queue gates the *prospective* merge.
   `main`'s push run still carries only `filter`, `renders` and the two
   cache primers (F3, unchanged by this unit and unchanged by Ev's
   ruling). Whatever reaches `main` by another route is ungated: a
   direct push, an admin merge with "bypass branch protections", the
   `[skip ci]` `STATUS.md` bot commit, and anything a queue run's own
   flake let through.
4. **Flake.** Under a queue a flaky test stops being a red someone
   re-runs and becomes an ejection plus a rebuild of everyone behind. The
   flake rate on a merge group is **unmeasured here** and is the number
   most likely to decide whether the trial is kept.
5. **Reading the red.** A failing queue run lives on a
   `gh-readonly-queue/…` ref and is not on the pull request's checks tab.

## THE ENABLEMENT RUNBOOK THAT WAS WRITTEN AND NEVER RUN — NOT AN INSTRUCTION

**Do not work through this. It is unrunnable on `evgunter/cad`**: merge
queue is not offered to a personal-account repository, so the "Require
merge queue" checkbox in *The settings* below does not exist to be
found, and Ev has ruled that the repository stays personal-account-owned
(top of this file). It is kept verbatim, imperative mood and all,
because it is the artifact the process failure produced — a nine-setting
runbook written for a screen nobody can open — and because a softened
copy would hide that. **Nothing on this page is a live step.**

**The step that is missing from it, and whose absence is this unit's
whole lesson: confirm the feature is available to the repository before
anything else.** `gh api repos/<owner>/<repo> --jq .owner.type` must
print `Organization`; on `evgunter/cad` it prints `User`. That one line,
run first, would have replaced everything below.

What follows was written for someone who had not read the pull request.
Everything in it is a repository setting; none of it is in the tree, and
nothing in this unit performs any of it.

### Before you start — as written; never executed

1. **`work/ciw/klint-row-still-sampled` must have landed.** Until then a
   queue run draws a k-lint row the PR run did not — see the dependency
   section. Check: `grep -n "KLINT_ROWS" scripts/ci-filter.py` and its
   `--selftest` line about the drawn dimension.
2. **This PR must be on `main`**, so that `ci.yml` carries the
   `merge_group` trigger and the `gate ok` job. Without the trigger, a
   queue dispatches `merge_group` and no workflow answers — GitHub's
   documentation: *"the merge will fail as the required status check
   will not be reported."*
3. **Look at a recent pull request run and confirm `gate ok` is green
   there.** It is the only required check, and it reports on pull
   requests as well as merge groups.
4. **Know what the flip tightens on the PULL REQUEST side**, because that
   is where it bites first and it is not a merge-queue property at all:
   requiring `gate ok` means an agent cannot merge a pull request whose
   gate is red — a real change to `CLAUDE.md`'s "agents merge their own
   PRs" — and, for the first time, **a failed render lane blocks a pull
   request**. That is decided and argued above on a measured post-`a5d9f41a`
   rate (8 failed render-lane jobs in 259 render-bearing runs, none at
   checkout). If it proves wrong, the lever is `render.yml`'s own failure
   semantics, not the required-check list.

### The settings — the screen this describes does not exist here

Repository → Settings → Branches → the rule protecting `main` (create
one if there is none; the API shows **no rulesets** on this repository as
of 2026-09-04, and classic branch protection was not readable from a
lane's token, so check by eye). The rule's branch-name pattern must be
literally `main`: *"A merge queue cannot be enabled with branch
protection rules that use wildcard characters (`*`)."*

| setting | value | why |
|---|---|---|
| Require status checks to pass | **on**, with exactly one check: **`gate ok`** | one name that reports on every tier; see the required-check section |
| Require branches to be up to date before merging | **off** | the queue is what makes the branch up to date; this setting fights it |
| Require merge queue | **on** | the switch |
| Merge method | **Merge commit** | `CLAUDE.md`: merge commits only, no squash, no rebase |
| Build concurrency | **5** | observed peak in flight is 4 over 24 h; at 4+ every PR's latency is exactly one run's wall clock. 1 produces a 22-minute median and a 63-minute worst case |
| Only merge non-failing pull requests | **on (Yes)** | with it off, a failing PR can ride into `main` behind a passing last PR — and because the change filter classifies a group against its parent, a docs-only last PR would skip the build entirely |
| Maximum pull requests to merge | **1** | keeps one merge ↔ one push to `main`, which the render re-baseline and `STATUS.md` regeneration assume. Costs nothing; raising it later is pure throughput |
| Minimum pull requests to merge | **1** | nothing ever waits for a group to fill, so the wait-time setting never fires |
| Status check timeout | **30 minutes** | the measured code-tier run is 528 s wall with a 1070 s maximum; 30 min is ~3× the worst observed and still short enough that a stuck group clears itself |

### Verify it works, in order — never performed, and unperformable

1. **Enqueue one docs-only pull request first.** This is the tier that a
   naive required-check list breaks, so it is the one to try first.
   Expect: a `merge_group` run appears in the Actions tab on a branch
   named `gh-readonly-queue/main/pr-<N>-<sha>`; most jobs report
   `skipped`; **`gate ok` is green**; the PR merges. Whole thing ≈ 1 min.
   If `gate ok` is pending forever, the queue is waiting for a check name
   that does not report — re-read the required-check list.
2. **Then one code-tier pull request.** Expect a full job set on the
   merge_group run: 12 `test (…)` jobs across two lanes and three eps
   rows, both `build + archive` jobs, both clippy rows, `k-lint (gate)`,
   and **no render lanes** (they are excluded from `merge_group` on
   purpose). Expect ~9 minutes from enqueue to landed.
3. **Then two at once**, to see a speculative group: the second PR's
   group contains the first PR's changes. Confirm the second group's
   `change filter` job prints a `base:` that is the first group's head,
   and `changed files:` that are only the second PR's.
4. **Read one queue run's `gate ok` log** and confirm it lists every job
   of the run with a conclusion. That listing is the whole of what the
   required check asserts.

### Turning it off in a hurry — moot: nothing was ever turned on

**The fastest safe stop is one checkbox.** Settings → Branches → the
`main` rule → **uncheck "Require merge queue"** → Save. Merging goes
back to what it is today, immediately; any pull requests sitting in the
queue are released and can be merged directly.

If merging is blocked and you are not sure which setting is doing it,
take them off in this order — each is independent and each is
reversible:

1. **Uncheck "Require merge queue".** This alone restores direct merges.
2. **Uncheck "Require status checks to pass"** (or remove `gate ok` from
   the list). Do this if PRs are blocked on a check that will not report
   — for instance if `ci.yml` stopped producing `gate ok`.
3. **As an escape hatch for one merge**, an administrator can use "Merge
   without waiting for requirements to be met (bypass branch
   protections)" on the pull request itself. It is per-merge and leaves
   the settings alone, and it is also a hole in the gate, so it is third.

**Do not** revert the `ci.yml` half to turn the queue off. The
`merge_group` trigger fires only when a queue exists and costs nothing
when none does, and `gate ok` is a reporting job that blocks nothing
unless a branch protection requires it. Removing them while a queue is
still configured is what produces a repository that cannot merge at all.

### If the trial is kept, what to write down — the trial never ran

The two numbers this trial exists to produce, neither of which any
measurement here could supply: **the observed merge latency** (against
the 528 s prediction) and **the rate at which merge groups fail for
reasons that are not the composed defect** — flake, and k-lint if it is
still drawing. Both belong in `docs/CI-MINUTES-2026-08.md`'s
public-runner block, beside the F3 re-costing.

## WHAT THIS UNIT COULD NOT DEMONSTRATE

Stated rather than implied, because a trial that has not run yet is
honest and a claimed demonstration is not. **The list is missing its
first entry and always was** — that this repository can have a merge
queue at all was never checked, and no entry below is worth anything
next to that one; see *THE PROCESS FAILURE* at the top. Every item here
now reads as permanently undemonstrable: no `merge_group` run will ever
exist on this repository.

1. **No `merge_group` run exists and none can until a queue is
   enabled.** Every statement here about what a queue run does is read
   off the guards, off GitHub's documentation and off the arithmetic —
   not off a run. That includes the central claim that the job set needs
   no `if:` edit: it is verified by reading every guard in the parsed
   workflow, which is a proof about the conditions, not an observation of
   a run.
2. **`gate ok` IS exercised, on a hosted run.** Run **33894380300**
   (head `0e16cc62`), job **101096934393**: green, and its log carries
   the whole job table and the line
   `gate ok: 32 jobs, all success, skipped or neutral.` — 33 jobs in the
   run, itself excluded. That is the first claim (every job terminal)
   and the second (every conclusion acceptable) both executing against a
   real 12-`test`-job code-tier run, with `actions: read` sufficing for
   the API read.

   **RE-TAKEN AFTER THE EXTRACTION**, because that receipt was of the
   inline heredoc. Run **33899974568** (head `2aa873c7`), job
   **101114375223**: green, `total_count` 33, the same 32-row table, and

   ```
   gate ok: 32 jobs, all success, skipped or neutral.
     tier: all   run_build: true   (29 success, 3 skipped)
   ```

   — so the extracted reader, the checkout and prune it now needs, the
   `contents: read` scope and the tier line are all exercised on a real
   code-tier run. `discipline` on the same run is green, which is the new
   `--selftest` step executing hosted. **The other six decision paths are
   re-driven by anything that runs `scripts/check-run-jobs.py
   --selftest`**, which both halves of CI do; they used to be driven once,
   by hand, before the logic went into the file, and by nothing after.
   What has **not** been exercised is `gate ok` under `merge_group`, for
   the same reason as 1.
3. **The latency table is a simulation**, on 138 real arrivals and two
   measured service times, with a server model taken from GitHub's
   documentation. Its assumptions: one median service time per tier
   rather than a distribution; arrivals produced under a no-queue regime,
   where a lane merges the moment it is green rather than enqueueing;
   and **no failures modelled**, which is the assumption the rebuild
   discussion above exists to qualify.
4. **Neither number models a recreated merge group.** A group rebuilt
   after an ejection or a queue jump gets a new ref and therefore a new
   run-level concurrency group, so the superseded run is not cancelled
   and finishes on the runners. That is unmodelled in the cost figure and
   in the latency table alike (above), and it cannot be measured without
   a queue.
5. **The runner-cost figure mixes two windows** — the merge rate from
   23.96 h, the per-run cost from the 5.5 h since un-sampling landed, in
   which only 9 code-tier runs have completed. n = 9 is thin, and the
   direction of the thinness is unknown.
6. **The repository's current branch protection was not readable.** The
   API returns 403 for a lane token on
   `repos/evgunter/cad/branches/main/protection`; the rulesets endpoint
   returns an empty list. So the runbook says what to set, not what will
   change.
