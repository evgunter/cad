---
id: f3-recosting-on-a-public-repo
kind: unit
title: F3 and the nightly demotions rest on an Actions allowance this repo no longer has: re-cost them on a public repo
status: review
opened: 2026-09-04
pr: 1796
branch: ciw/f3-recosting
needs_ev: true
---



Opened on Ev's direction (in-chat, 2026-09-04: CIW opens the re-costing
as a unit) out of the closure of `main-latently-red-at-tier-all`, whose
class half this is.

## The fact that moved

`evgunter/cad` went **public** on 2026-09-03 (the repository's
`visibility` is `public`; the tree's own record is `5cc16e81` through
`483212ef`, and `483212ef` restates the runner spec). Two things follow
and neither is a matter of opinion:

- **Standard-runner minutes are free.** `docs/CI-MINUTES-2026-08.md`
  opens with *"the Actions allowance was being consumed faster than the
  work justified"*. That sentence is the premise of the whole document
  and of every trim it licensed.
- **The runner is 4 vCPU / 16 GB**, up from 2 vCPU / 7 GB. `483212ef`
  says so at the site and marks the document's timings as predating it.

The same day, and before the visibility change, the account's Actions
spending limit denied job starts outright for two and a half hours
(`work/issues/actions-budget-denies-job-starts`, closed). That is the
old regime's last data point, not this one's.

## What is therefore open

Three ratified or landed decisions were bought with billed minutes and
have not been re-read since the price went to zero:

1. **F3** — a `push: main` run is reduced to `filter` +
   `rebuild-latency` + `renders`, skipping build, test, clippy and
   k-lint (`docs/CI-MINUTES-2026-08.md` §F3, commit `0768882`, Ev
   authorised 2026-08-20). Its stated cost is that *"the landed main
   commit is then never itself tested"*, and the compensating control
   is the next PR's merge ref. The scheduled full run on main that
   would have paired with it was declined by Ev on 2026-08-22, on the
   same cost grounds.
2. **This month's demotions to the nightly** — TCOST-C1, C2 and C3 move
   `corrupt input (release profile)`, the rustdoc gate's excluded roots
   and third pass, and the python suite's ungated re-take out of the
   per-PR gate. Each argued a billed-minute saving.
3. **The declines that named a minute as the reason**, chief among them
   `doc-gate-two-unread-axes` axis (b): a `--release` doc pass was
   declined because it is a fourth compilation on a job F6 had fought
   back inside two billed minutes.

## What this unit does, and what it does not

**Measures first.** What a restored full `push: main` run now costs in
**wall clock** on the 4-vCPU runner — not in minutes, which are no
longer the currency — against what it buys: the landed commit tested as
landed, rather than a merge preview of it. The build jobs are the
critical path (F4: `build + archive (interval)` alone is ~88% of a
13.75-minute critical path, at 2 vCPU), so the re-take is a re-take:
every figure in `CI-MINUTES-2026-08.md` predates the runner change and
none of them may be quoted forward.

**Then proposes, on an `[ev]` PR.** F3 is Ev's ruling and the declined
scheduled run is Ev's decline; neither is reopened by a lane deciding
it now costs less. This unit's deliverable is the measurement plus a
recommendation, and the change — if any — lands after Ev answers.
`work/README.md`: the question rides an `[ev]` PR and this item sets
`needs_ev` when it is asked.

**Out of scope**, so that the unit does not become the whole board:

- Cache and build knobs stay S-TCOST's under this program's `keep_out`,
  including the finding that makes the wall-clock number worse than it
  looks (`work/tcost/rust-cache-never-restores-across-branches`). This
  unit may cite that measurement; it may not fix it.
- The change filter's tiering is not re-opened wholesale. F3 is one
  decision with one written argument, and that is the subject.
- Nothing here reads on whether the demoted rows WORK — that is
  `work/ciw/nightly-demotions-have-never-run`, and it is a different
  defect with a different fix.

## Inherited class: "2 vCPU" asserted in prose, 2026-09-04 (from unit 5)

Unit 5 (`perf-history-cannot-identify-its-host`, PR 1722) edited six
files and found that each still asserted the old runner three lines from
the paragraph it had just added saying the runner changed. It fixed the
six **in its own diff** and stopped there; the rest is this unit's,
because the subject is the same one — what the 2026-09-03 runner change
invalidates.

**The count, measured at that PR's merge base.** `2 vCPU` or `2-vCPU`
appears **51 times across 43 files**, excluding the two occurrences in
`docs/perf-data/criterion/README.md` that correctly *describe* the change.
It is not one sweep but three sub-classes, and only the third is likely to
be a mechanical edit:

* **~26 hits, `crates/*/Cargo.toml` + `crates/*/tests/all.rs` (13 crates,
  one pair each).** These cite the 2-vCPU runner as the *reason* for the
  one-test-binary layout ("on the CI runner (2 vCPU) the per-binary
  codegen+link…"). **The number is load-bearing on a decision**, so these
  are not text fixes: the layout's justification has to be re-checked at
  4 vCPU / 16 GB before the sentence is rewritten, and re-checking it is
  this unit's kind of work.
* **~10 hits in costing prose** — `docs/CI-MINUTES-2026-08.md` (×3),
  `docs/GENERICS-BUILD-COST.md` (×2), `docs/PERF-SCAN-2026-08.md`,
  `.github/workflows/ci.yml` (×5), `.github/workflows/nightly.yml`, plus
  this item's own two. Same status as every other figure in
  `CI-MINUTES-2026-08.md`: predates the change, not quotable forward.
* **the remainder** — `memories/perf-measurement-lane.md:25`,
  `scripts/doc-gate.sh`, `scripts/check-ci-mirror-parity.py`,
  `benches/benches/kernel.rs`, a few `crates/*/tests/*.rs` headers and
  three `work/` logs. These are variance/fat-tail asides where the vCPU
  count is incidental to the point; unit 5's repair spelling ("a shared
  hosted runner has a fat tail") drops the stale number without
  claiming a new one, and it applies unchanged here.

`memories/perf-measurement-lane.md:25` is worth calling out on its own:
it is the file the perf READMEs cite as their authority, so it is the
one whose staleness propagates.

Recorded here, not fixed by unit 5, because a 43-file sweep inside a
two-field emitter change would have buried the change it was reviewed
for — and because the first sub-class is a re-costing question, which is
this unit's whole subject.


## Evidence: the sharpest observed instance of F3's residue (2026-09-04)

`work/lib/pncad-py-tag-inventory-misses-two-measure-tags` is not merely
an example of "the landed main commit is never itself tested". It is
that residue producing a red that **no PR run anywhere could have
caught**, which is a stronger claim and is established rather than
argued:

- M10 (PR 1685) added two `node_error_tag` values, merged 17:48 UTC on
  2026-09-03. Its run had the values and no gate reading them: green,
  correctly.
- LIB (PR 1696) added the `TAG_INVENTORY` gate that reads them, merged
  17:55 UTC — seven minutes later. It last merged `main` at 16:31, so
  its branch never contained M10's values (`461e0f9a` is not an
  ancestor of its final head). Its run had the gate and no values:
  green, correctly.

The composition existed for the first time on `main`, and `main`'s push
run classified docs-tier and skipped both test rows. So the first tree
to execute the composed state was an unrelated branch's merge ref, and
the cost has been billed to every code-tier PR since — seven branches
in one night, measured, before anyone routed it.

**What this is worth to this unit.** F3's written cost is "when main
moves between a PR's last run and its merge — frequent at this repo's
merge rate — that exact combination went untested", with the next PR's
merge ref named as the compensating control. This instance shows the
control working exactly as designed and still being the wrong shape:
it detects, but it detects on a third party's branch, days late, with
no route back to either author, and it charges the triage to whoever
happens to draw the point. Any proposal this unit makes about what a
`main` push run should re-gate has to be measured against this case,
not against the abstract residue.

## THE MEASUREMENT (2026-09-04, CIW unit 8)

Every figure below is read from the hosted Actions API on 2026-09-04 and
names the run or job it came from.

**The population, stated so it reproduces.** `pull_request` runs of
`ci.yml` created between `2026-09-03T15:20Z` and `2026-09-04T05:47Z`
with `status=completed` **and `conclusion` in {`success`, `failure`} —
cancelled runs are excluded**, because a cancelled run's job set is a
truncation of the thing being measured rather than a sample of it. That
is **220 runs, of which 149 are code-tier** (their `build + archive` job
is not skipped). Keeping the cancelled runs in gives 268 / 195 and
shifts every duration downward; the exclusion was applied throughout and
was not stated in the first revision of this section, which is a
reproducibility defect and is corrected here. The cancelled runs are not
discarded from the argument — they are the subject of M3.

Nothing from `docs/CI-MINUTES-2026-08.md` is quoted forward; where a
figure of its is named it is named as the thing being replaced.

### M1 — the runner, read first-hand rather than taken from prose

Run **33830873453**, job **100893490483**, step `runner + linker
provenance (for perf attribution)`: `nproc` prints **4** and `free -g`
prints **15** GB total. `483212ef`'s "4 vCPU / 16 GB" is confirmed at
the runner, not just at the comment. The same job's rust-cache step
prints `Cache hit for: v0-rust-build-default-Linux-x64-fa41882e-fd5fb1c1`
… `full match: true`, 263 MB — which is the key and the hit, but not
proof of cross-branch inheritance; the last section says why, and which
job does prove it.

### M2 — what a full code-tier run costs on that runner

| figure | value | n |
|---|---|---|
| run created → last job end | **442 s (7.37 min)** median; p25 390, p75 480, max 1086 | 149 |
| first job start → last job end | 437 s median | 149 |
| the same, **`tier=all` runs only** (`--workspace`; the `interval backend crate` job ran), created → last job end | **482 s (8.03 min)** median, p75 512, max 1086 | 66 |
| `tier=all`, first job start → last job end | 476 s median | 66 |
| `build + archive (default)`, tier=all | **336 s** median (min 237, max 695) | 23 |
| `build + archive (interval)`, tier=all | **388 s** median (min 131, max 410) | 43 |
| `build test binaries + archive` step alone, all tiers | 296 s median, p75 342, max 649 | 162 |
| live jobs per code-tier run | 15 median (11–18) | 149 |
| **job-seconds per code-tier run** | **1462 s = 24.4 job-minutes** median | 149 |

`tier=all` is `CARGO_SCOPE=--workspace` (`scripts/ci-filter.py:695`), so
the two build rows are like-for-like with the `--workspace` figures the
2-vCPU readings used: **820 s / 840 s cold and 603–677 s warm** become
**336 s / 388 s**, and the **13.75-minute** critical path becomes
**7.9 minutes**. Both the core count and the cache priming that landed
2026-09-03 are inside that difference and this unit does not separate
them; the number that matters for the question is the total.

**A push run is estimated by a PR run, and here is why that is fair.**
On a merge commit the change filter reads the merge's own diff — the
PR's diff — so the tier, the cargo scope and the job set are the same
ones the PR run had. The one systematic difference makes the estimate
conservative: a push run on `main` restores `main`'s own cache entry
under an exact key every time, where the PR population mixes hits and
misses.

### M3 — the merge rate, and the cancellation that decides the question

200 `push` runs on `main` over **45.66 h** = **4.36 pushes/h**. Of those
200, **90 (45 %) are code-tier** — measured as "the `renders` job was
not skipped", which on a push means `RUN_K_LINT=true`, which
`scripts/ci-filter.py:1730` sets for every tier except `docs`. So
**1.97 code-tier merges an hour**.

Median gap between consecutive pushes: **308 s**. Only **32 of the 90
code-tier pushes (36 %) have ≥442 s before the next push arrives.**

`concurrency: cancel-in-progress: true` on `${{ github.workflow }}-${{
github.ref }}` (ci.yml:103–105) therefore cancels most of them, and the
right way to see that is not the aggregate. **51 of the 90 code-tier
push runs (57 %) are already `cancelled` today** — at a job set whose
median is **259 s** (run created → updated), against 40 s for a docs-tier
push run, where only 16 of 110 (15 %) are cancelled. 67 of 200 (34 %)
overall is the mix of those two, not a rate that applies to anything.

The direction matters and the first revision of this section had it
backwards. The cancellation rate is not evidence that pushes arrive
faster than a 40-second job set; it is evidence that **the longer a push
run is, the more of them die**, and that relationship is already
measurable across the only two run lengths this repo has: 15 % at 40 s,
**57 % at 259 s**. A 442 s run sits above both, and the direct estimate
of its survival is the 36 % above.

**This is not a hypothetical, and it lands on the exact evidence this
item rests on.** The push run for LIB's merge — the commit that composed
the defect — is run **33787453014**, started `17:55:42Z`, **cancelled at
`17:59:48Z`** (246 s) because the next push's run **33787719180**
started at `17:58:23Z`. A restored full run needs ~442 s to a verdict
and **348 s** to red the first test row — that offset is not a guess:
job `100761051102` on run `33788618577` completed **+348 s** into its
own run (`started_at`/`completed_at` against the run's `created_at`).
**Restoring the job set alone
would not have caught this defect: the run would have been cancelled
mid-build.** Any proposal here is a proposal about the concurrency group
as much as about the job list.

### M4 — what it would cost to run, if it were allowed to finish

1.97 code-tier merges/h × 1462 job-seconds = **+48 job-minutes an hour**,
i.e. **+0.80 mean concurrent jobs**.

Against what: over the same window, completed PR runs alone hold a
time-weighted mean of **4.3** concurrent jobs, p90 **12**, **peak 36**
(a lower bound — cancelled and in-progress runs are not in that
population). Queue delay today, run created → first job start, is
**median 3 s, p90 12 s, p99 25 s, max 304 s** over all 220 runs. There
is no queue to speak of, and +0.8 does not make one.

**In money: zero.** `visibility: public`, `private: false` on the
repository record read 2026-09-04. Standard-runner minutes are not
billed, which is the premise change this unit exists for.

### M5 — the narrow variant, priced

Restricted to `change filter` + `build + archive` + the `test` rows —
the rows that execute the composed tree — a code-tier run is:

* **477 job-seconds = 8.0 job-minutes**, i.e. **33 %** of the load;
* **433 s** first-start-to-last-end against the full set's 437 s — **99 %
  of the same wall clock**, because the build is the critical path and
  the tests hang off it. The "build is ~88 % of it" line survives the
  runner change as a shape even though its number does not — and it is
  at `docs/CI-MINUTES-2026-08.md:49`, in *The code-tier run*, not in §F4
  where this item first cited it.

So narrowing buys two thirds of the runner load back and nothing else.

### M6 — the chain in the Evidence section above, verified against the tree

Re-derived here rather than taken:

| fact | check |
|---|---|
| M10's two `node_error_tag` values | `5a3fc838`, merged as `461e0f9a` at **17:48:09Z** |
| LIB's `TAG_INVENTORY` gate | `434964df`, merged as `bdfa604b` at **17:55:39Z** — 7 m 30 s later |
| LIB's branch never held M10's values | its last main merge is `4f67c3b6` at **16:31:05Z**; an ancestry check of `461e0f9a` against `bdfa604b^2` is **false** |
| the composition first existed on `main` | both PR runs were green, correctly |
| the repair | `3b038f9a`, landed on main as `e75d68bd` at **2026-09-04T02:06:54Z** — **8 h 11 m** after the composing merge |

**And one thing in that section is wrong, in this file and elsewhere.**
It says `main`'s push run "classified docs-tier and skipped both test
rows". The first half is false. Run **33787453014** ran `renders`, which
on a push requires `RUN_K_LINT=true`, which is set for every tier but
`docs` (`scripts/ci-filter.py:1730`); `RUN_BUILD` is set by the same rule
one line up (`:1713`), so that push had `run_build=true` and skipped the
build and test rows **purely on F3's `github.event_name != 'push'`
guard**. 45 % of main's push runs are code-tier (M3) and 100 % of them
skip the test rows. The tier is not what does it. F3 is what does it,
which makes the instance stronger evidence about F3 than the file
claimed, not weaker.

### M7 — what F3's compensating control actually cost, measured

Between the composing merge (17:55:42Z) and the repair (02:06:54Z),
**83 completed PR runs** were created. **42 of them, across 20 distinct
branches, have a failing `test` row** — 53 of the 54 failing test jobs in
that window are shard `2/2`, the signature the closed issue records. The
branches: `ciw/render-lane-merge-ref` (5), `tcost/k3-unit` (5),
`ciw/perf-host-identity` (4), `fix/band-linear-sweep` (3),
`tcost/11-aggregation-guard-home` (3), `tcost/k2-unit` (3), and 14
others. The item's "seven branches" was the count of the ones then
listed; the measured figure is 20.

**Detection was fast. Attribution never happened.** The first red is run
**33788618577** on `tcost/k2-unit`, created `18:07:23Z` — **11 m 41 s**
after the merge that caused it — and it went red at `18:13:11Z`, **17 m
29 s** after that merge (job `100761051102`, completed +348 s into its
run). F3's control did not fail and it was not slow; it named a
stranger. The 8 h 11 m is the distance from a red nobody owned to a
repair, and the 42 runs on 20 branches is what was billed to lanes that
had written none of it.

That is the cost to weigh: **not detection latency — attribution.**

### M7a — the head start, corrected, and it runs the other way

**The first revision of this reading was wrong by about 3× and stated
backwards, and correcting it makes the case for restoring the gate
stronger rather than weaker. Saying so is the point of this section.**

It compared **11 m 41 s** — the moment the PR run was *created* — with
**7.4 min**, the *duration* of a push run. Those are two different
clocks, and subtracting one from the other produced "PR runs still red
first, by about four minutes", which asserts the opposite of what the
data says.

Both clocks anchored on the merge at `17:55:42Z`:

| | when it reds | after the merge |
|---|---|---|
| a restored push run on `bdfa604b` | 17:55:42 + 348 s = **18:01:30Z** | **5 m 48 s** |
| the same run charged its full median critical path | 17:55:42 + 442 s = 18:03:04Z | 7 m 22 s |
| the PR run that actually found it | **18:13:11Z**, measured | **17 m 29 s** |

**The push run reds first, by 11 m 41 s red-to-red** (or 10 m 07 s if
the push run is charged the whole run rather than the moment its test
row reds). The 11 m 41 s appears twice in this document for a reason
that is arithmetic and not coincidence: both runs carry the same in-run
offset, so the gap between their reds is exactly the gap between their
starts.

**What is estimated and what is measured.** The 348 s is measured, on
run `33788618577`, whose diff was TCOST's `geom-core/src/spline/` work —
a different cargo scope from LIB's merge, so the push run's own offset
would differ. It is the offset of a comparable code-tier run, not a
measurement of the counterfactual, and nothing here can be. What is not
an estimate is the direction: the push run starts **11 m 41 s** earlier,
and no plausible scope difference closes that.

**So the section above this one is overstated where it says the control
"detects on a third party's branch, days late".** The branch is right and
the days are wrong: 11 m 41 s, measured. Everything the sentence was
reaching for survives the correction and lands harder — the control is
prompt, and it is still the wrong shape, because being prompt is not the
property it was missing.

### M8 — the scheduled run, re-priced, and it does not come back

Ev declined it on 2026-08-22 on cost. At today's prices:

* **hourly**: 24 runs/day × 24.4 job-min = **9.8 job-hours/day** — this
  is *cheaper* than the per-merge run below (19.2 job-hours/day), because
  it does not scale with the merge rate;
* **nightly**: 24.4 job-minutes/day, and up to 24 h of latency.

But the decline's reasoning was never only cost, and the part that was
not cost is confirmed by M7: **the next PR's merge-ref does discover the
fact, in 11 minutes.** A scheduled run buys that same second discovery,
later, and names a *window* of merges — 4.4 of them per hour at the
measured rate — so it does not fix attribution either. **The price
change does not reopen the scheduled run.** Recommend leaving that
decline standing on its own grounds.

## THE OPTIONS, WITH NUMBERS

| option | cost | latency to a verdict | attribution |
|---|---|---|---|
| option | cost | when the composed defect reds | attribution |
|---|---|---|---|
| **A. full job set on `push: main`, + a concurrency design pass** | **+48 job-min/h (+0.80 mean concurrent jobs), plus the design pass below; $0** | **5 m 48 s** after the merge (348 s in-run offset); 7 m 22 s if charged the full run | the merge commit, hence both PRs |
| **B. as A but test rows only** | +16 job-min/h (+0.26), same design pass; $0 | same offset — the tests are what red | same |
| **C. scheduled full run** | 9.8 job-h/day hourly; 24.4 job-min/day nightly | ≤1 h / ≤24 h | a window of ~4.4 merges (hourly) |
| **D. do nothing** | 0 | **17 m 29 s**, measured | a stranger's branch; 42 red runs on 20 branches in the one observed instance |
| **A or B with the concurrency left alone** | as above, minus the 57 % that die | on the **next** merge's push run, which carries this tree too | the burst, not the commit — ~2 merges at a 308 s gap |

## RECOMMENDATION

**Option A.** Restore the full job set on `push: main` — that is, revert
F3's `github.event_name != 'push'` guards — **and stop cancelling push
runs, which is a design pass and not a line (next section).**

**The number: +48 job-minutes an hour of runner time, +0.80 mean
concurrent jobs against a measured mean of 4.3 and a queue delay of
3 seconds, and $0.** The composed defect reds **5 m 48 s** after the
merge, on the merge commit — **11 m 41 s before** the innocent PR run
finds it (M7a).

**A without the concurrency half is weakened, not void, and the first
revision of this section overstated that as "measurably worthless".**
57 % of code-tier push runs die today and a 442 s one would do worse
(M3) — but a cancelled push run's tree is not unexamined: the **next**
merge's push run contains it, and reds on it. What degrades is
attribution, from "this commit" to "this burst" — about **two** merges
at a 308 s median gap. That is still sharper than option C's window of
~4.4, which this document rejects on exactly that ground, so the
comparison has to be stated this way round or it is inconsistent. The
concurrency half buys the difference between two candidates and one; it
does not buy the difference between something and nothing.

**Why the full set rather than B.** B is a third of the load, and the
load is not the binding constraint — nothing queues today, and the
difference between +0.80 and +0.26 concurrent jobs is not a difference
anyone can observe. What B gives up is every composition that is not a
test failure, and this unit has exactly **one** observed instance to
generalise from. Buying the coverage at a cost nobody can feel is the
better trade; B is the answer if the footprint matters more than the
coverage — it differs from A only in which `if:` guards are reverted,
and it carries the same concurrency design pass.

### The concurrency half is three mechanisms, and it needs its own design pass

The first revision priced it as "a per-SHA group, one line". That is
wrong, and the interactions are in the tree rather than hypothetical:

1. **`render.yml` has a concurrency group of its own, keyed on the
   caller's ref.** `render.yml:268–275`: `group: render-demos-${{
   inputs.gate && 'gate' || 'dispatch' }}-${{ inputs.ref || github.ref }}`
   with `cancel-in-progress: ${{ inputs.gate == true }}`. Inside a
   `workflow_call` the github context is the caller's, so on every push
   to main that group is the same string — `…-gate-refs/heads/main`.
   Today it never fires, because `ci.yml`'s run-level group takes the
   whole older run down first. Give push runs a per-SHA group at the run
   level and it **starts** firing: the older run survives, its `renders`
   job is cancelled by the next push's render, and the run concludes
   `cancelled` — **on precisely the merges the change exists for.** So
   the render group has to move to the caller's SHA in gate mode too,
   and that is an edit to a workflow with a second (dispatch) caller
   whose behaviour must not change.
2. **`ci.yml:1830–1840` argues `cache-on-failure: false` FROM push runs
   being cancelled** — "this workflow's concurrency cancels an
   in-progress run when main moves again, so a rotation's ~13-minute
   build can be cancelled part-way". Remove the premise and that
   argument needs re-reading; it may survive on the failure case alone,
   but nobody has read it that way. Cache is S-TCOST's fence, so this
   unit names it and stops.
3. **The write side-effect assumes serialisation.** `renders`'
   `push_to` (ci.yml:4203) commits render re-baselines to `main`;
   concurrent push runs mean two lanes committing to `main` at once.
   (The rebuild-latency history append is **not** a second instance of
   this — it left ci.yml for the nightly on 2026-08-22, ci.yml:3450 — so
   there is one such write here, not two.)

**None of that is priced above and this unit does not pretend to price
it.** The +48 job-min/h is the cost of the *jobs*. The concurrency half
is a design question about three interacting mechanisms, and the honest
statement to put in front of Ev is that it is a second unit, not a line
item in this one.

**What A does NOT buy, stated plainly.** It does not remove the 42 red
runs: PR runs still red on a broken `main`, ~11 m 41 s later (M7a), and
a branch that had already started is not rescued by a red that lands
after its own build began.
What A buys is that the red exists **somewhere that names its author**,
and that it exists **first**. Whether that shortens the 8 h 11 m
depends on someone reading a red push run on `main`, and **no instrument
for that has ever existed here** — there is nothing in this measurement
that proves it would be read. That is the honest weak point of the
recommendation and it is the thing to weigh against +48 job-min/h.

## WHAT THIS UNIT COULD NOT MEASURE

1. **Whether an attributed red is repaired faster than an unattributed
   one.** The 8 h 11 m of M6 is the only observation and it is from the
   current regime. No counterfactual is available.
2. **The false-positive rate of a push gate.** 28 % of code-tier PR runs
   outside the TAG_INVENTORY window are red, but those are branches
   under development; a merged commit was green on its own branch, so
   that rate does not transfer. The flake rate on `main` specifically is
   unmeasured.
3. **Cold-vs-warm on the current runner, separated.** One restore line
   was read first-hand (M1). The build medians in M2 mix both states,
   which is why they are quoted as what a run costs and not as a cache
   reading. Cache is S-TCOST's fence.
4. **Whether A makes `cache-prime` redundant on code-tier pushes.** The
   build job carries the same `shared-key` and rust-cache does not
   re-save on an exact hit, so on the face of it A subsumes the primer
   for the 45 % of pushes that are code-tier — but the primer also
   covers the other 55 %, and that is S-TCOST's call, not this unit's.
   Flagged, not touched.

## ONE FINDING OUTSIDE THIS FENCE, REPORTED NOT FIXED

`work/tcost/rust-cache-never-restores-across-branches` says "F3 means
`main` never runs the build job at all… so no PR can inherit a build
cache from anywhere". The first clause is still true; **the second is no
longer**. `cache-prime` / `cache-prime-interval` landed on `push: main`
on 2026-09-03 and write under the default branch's scope.

**The evidence that proves inheritance, and the evidence that does not.**
The first revision cited job `100893490483`
(`Cache hit for: v0-rust-build-default-Linux-x64-fa41882e-fd5fb1c1`,
263 MB, `full match: true`) — which shows the key and the hit, but that
job is the **fourth** run on its own branch, so the entry it restored
could have been its own save. It does not prove cross-branch
inheritance. What does: run **33827576986**, the **first-ever** `ci.yml`
run on branch `m10/hotfix-tag-inventory` (the branch has two runs; this
is the older), job **100883473006**. Its build compiled only the
workspace crates and a handful of registry crates and printed
`Finished 'test' profile … in 54.22s`, and its post step printed
`Cache up-to-date.` — rust-cache's exact-hit, nothing-to-save line. A
cold build of that scope recompiles ~225 registry crates and takes
minutes, and this branch had never run before, so the entry came from
`main`'s scope. **That is the reading `docs/CI-MINUTES-2026-08.md`'s
cache section asks for** ("the first PR opened after this merges, first
run, `build + archive`'s restore line"), and it says the primer works.
S-TCOST's item to update; cited here, not edited.

## FIX PASS (2026-09-04) — what a verification lane found, and what it changed

A verification lane re-derived every load-bearing figure above from 760
`ci.yml` runs. They reproduce, most to the digit. What did not survive
was **prose**, in the two places this unit was warned about hardest: a
number stated backwards, and a scope stated too small. Both are
corrected in place above rather than appended as errata, and both are
named here so no reader has to diff two revisions.

**The corrections, each with the section that now carries it:**

1. **The head start was wrong by ~3× and pointed the wrong way** (M7a,
   new). "PR runs still red first, by about four minutes" compared a run
   *creation* time with a run *duration*. Measured: the PR run reds at
   `18:13:11Z`, **17 m 29 s** after the merge; a push run reaches the
   same in-run offset (348 s, job `100761051102`) at `18:01:30Z`, **5 m
   48 s** after it. **The push run reds first, by 11 m 41 s.** The error
   ran against this unit's own recommendation, which is exactly why it
   survived a self-review: a number that makes your case worse does not
   trip the reflex that checks it.
2. **The concurrency half is three mechanisms, not a line** (new section
   under RECOMMENDATION): `render.yml`'s own gate-mode group on the
   caller's ref, which *starts* firing once the run-level group goes
   per-SHA and cancels the render job on exactly the merges the change
   is for; the `cache-on-failure: false` argument at ci.yml:1830–1840,
   which is argued *from* push runs being cancelled; and `renders`'
   `push_to` write to `main`, which assumes serialisation. Priced as
   "one line" before; now stated as a design pass this unit does not
   price. (One correction inside the correction: the rebuild-latency
   history append is **not** a second serialised write — it moved to the
   nightly on 2026-08-22, ci.yml:3450.)
3. **"Measurably worthless" was an overstatement that made this document
   inconsistent with itself** (RECOMMENDATION). A cancelled push run's
   tree is carried by the next merge's push run, so attribution degrades
   to the burst — ~2 merges at a 308 s gap — which is *better* than the
   ~4.4-merge window this same document rejects option C for. A without
   the concurrency half is weakened, not void.
4. **The population did not reproduce as written** (top of THE
   MEASUREMENT). Cancelled runs were excluded throughout and that was
   never said; as written it reads as 268 runs / 195 code-tier and every
   duration moves. The exclusion is now part of the definition.
5. **A third confirmation this unit had and did not use** (M3): **51 of
   90 code-tier push runs (57 %) are already cancelled**, at a 259 s
   median job set, against 15 % of docs-tier pushes at 40 s. The
   aggregate 34 % was quoted with its causality backwards.
6. **Two citations repaired**: the "~88 %" line is at
   `docs/CI-MINUTES-2026-08.md:49`, not §F4; and the cache "first
   reading owed" is proved by job `100883473006` on run `33827576986` —
   a branch's **first-ever** run, 54.22 s build, `Cache up-to-date` —
   not by the fourth run of a branch that could have restored its own
   save.

Nothing in the measurement moved. The recommendation is unchanged and
its central number is stronger than it was written: **5 m 48 s to a red
on the merge commit, 11 m 41 s ahead of the stranger who pays for it
today.**
