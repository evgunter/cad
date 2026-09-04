---
id: f3-recosting-on-a-public-repo
kind: unit
title: F3 and the nightly demotions rest on an Actions allowance this repo no longer has: re-cost them on a public repo
status: review
opened: 2026-09-04
pr: 1796
branch: ciw/f3-recosting
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

## EV'S TWO QUESTIONS (PR 1796, comment 5537281174, 2026-09-04 07:35Z)

### The enumeration, and how it was built

Both answers rest on a population of **recorded main-red instances**, so
the search has to be describable. It is: `git grep` over `origin/main`
for `main is red|main … non-compiling|latently red|reaches main|red on
main` across `work/**.md` and `docs/**.md` — 13 files — read in context,
keeping the ones that record **a defect present on `main` that no run on
`main` executed**. Five qualify:

| # | instance | mechanism | point-dependent? |
|---|---|---|---|
| 1 | `TAG_INVENTORY` two tag values (`bdfa604b`, 17:55:42Z) | **composition** of two green PRs | **no** — fires at both lanes and all three eps rows (its own closed item's evidence) |
| 2 | `MateFault::Unleverable` non-compiling (`50d9ba21`, 05:26:52Z) | **composition** of two green PRs | **no** — measured red on both the default and interval clippy rows |
| 3 | `main-latently-red-at-tier-all` (pyo3 wheel, doc collision) | one PR's own rows **unexecuted at its tier** | no |
| 4 | `probe-census-red-interval-cfg-gate` | **a sampler** — but `k-lint`'s 1-of-5 row draw, not the lane/eps draw | (k-lint row, not a config point) |
| 5 | `pncad-py-python-feature-clippy-lane-is-red` | a configuration **no CI row runs at all** (`--features python`) | no — a coverage hole, not a miss |

**What this population cannot contain**, and it is the crux of Q2: a
defect that only shows at a configuration point nothing drew is
invisible until something draws it. The enumeration is of defects
somebody wrote down, which biases it toward defects that cost enough to
be written down. Absence of a lane/eps-draw red here is evidence that
nothing is looking, not evidence that none exist.

### Q1 — "for (B), do we have historical evidence on how much that vs (A) would've caught things?"

`(A)`/`(B)` are read as the options table's rows: **A = the full job set
on `push: main`; B = the test rows only.** (The adjacent question — the
job set with or without the concurrency change — is answered by the same
numbers and is below.)

Two ingredients, both measured:

**How much clear air each composing merge had** — time from its push run
starting to the next push run starting, which is when
`cancel-in-progress` fires:

| composing merge | push run | clear air | the run actually lived |
|---|---|---|---|
| `bdfa604b` (instance 1) | `33787453014` | **161 s** | 246 s |
| `50d9ba21` (instance 2) | `33840520387` | **95 s** | **102 s** |

**How fast each row reds on the defect in question**, as an offset into
its own run:

| row | reds at | source |
|---|---|---|
| `clippy` (default) | **+84…96 s** | 3 runs in the 34-minute non-compiling window |
| `clippy + doc-tests (interval)` | **+75…92 s** | same 3 runs |
| `build + archive (default)` | +127…134 s | same |
| `build + archive (interval)` | +155…202 s | same |
| `test (…, 2/2)` on instance 1 | **+348 s** | job `100761051102`, run `33788618577` |

Job `100924046762` (run `33841298810`, `clippy`) is read first-hand:
`error[E0004]: non-exhaustive patterns: &editor_core::MateFault::Unleverable
{ .. } not covered`, `crates/viewer/src/tree.rs:317:11`, exit 101 at
+84 s. That is instance 2, on a stranger's branch, four times over.

**The counterfactual, n = 2:**

| | instance 1 | instance 2 | caught |
|---|---|---|---|
| **A** (full set), cancellation as today | no — +348 s into a 246 s life | **yes, by 6–27 s** — `clippy` at +84…96 s into a 102 s life | **1 of 2** |
| **B** (test rows only), cancellation as today | no — same +348 s | no — its earliest row is the build at +127…134 s | **0 of 2** |
| **A + no cancellation** | yes, +348 s | yes, +84 s | **2 of 2** |
| **B + no cancellation** | yes, +348 s | yes, +127 s | **2 of 2** |

**So the historical evidence says A strictly dominates B, and the margin
is real in one of the two cases.** The row that catches instance 2
fastest — and the only row that catches it at all under today's
cancellation — is `clippy`, which **B does not restore**. This is the
abstract risk the first revision named ("what B gives up is every
composition that is not a test failure") turning up in the record one
night later, and it moves the recommendation from a judgement call to an
observation.

**Do not over-read n = 2.** Two instances is two instances; instance 2's
margin is 6–27 seconds on a run that lived 102 s, which is a scheduling
coin-flip and not a property. What the table supports is an ordering
(A > B, both ≫ nothing) and one hard fact — **neither variant catches
instance 1 without the concurrency change**.

**Instances 3–5 are not in the table because A does not address them,
and saying so is part of the answer.** Instance 3 is a *tier* question:
a restored push run classifies at the merge's own tier, so it skips
exactly the rows that PR's own run skipped. Catching it needs push runs
at `tier=all`, which is a different and more expensive proposal than
anything priced here. Instances 4 and 5 are a different sampler and a
missing row respectively.

### Q2 — un-sampling the configuration draw

Ev: *"CI is weakened right now because of sampling only certain
configurations… undoing that sampling now that actions minutes are much
cheaper is probably a good idea regardless of if we still need the fix
described in this pr."*

**Q2.1 — attribution: how many main-reds are the draw's?** Of the five
recorded instances above, **zero** are attributable to the lane/eps
draw. Two are compositions, one is tier scope, one is `k-lint`'s
row sampler, one is a configuration with no row at all. And the two
compositions are **point-independent**: instance 1's own record has it
firing at both lanes and all three eps rows, and instance 2 is measured
red on both lanes' clippy rows — so un-sampling would not have caught
either of them one second sooner.

That is not a verdict against un-sampling, because of the bias stated
above. It is a verdict against *justifying* un-sampling by the recorded
reds: **the record cannot answer Ev's "how much of the reds on main are
from that vs conflicts", and the reason it cannot is the same reason the
draw is worth undoing.** The honest ground is the price, which is small
(below), and the exposure, which is measurable: a merge is gated at
**one** of six points, and each point gates between 9 % and 31 % of
code-tier runs (observed frequencies: interval/1e-12 31 %, default/1e-12
22 %, interval/default 15 %, interval/1e-6 13 %, default/default 10 %,
default/1e-6 9 %; lane marginal 59 % interval, eps marginal 54 % 1e-12).
Those are gating frequencies, not a claim about the hash: lanes can also
be *asked for* by trailer, and `CONFIG_SOURCE` separates drawn from
requested in a run's log, which this reading did not open.

**S-TCOST's census could not be used.** There is no `docs/S-TCOST-LOG.md`
on `origin/main` and no `nontest_failures.json` in the tree — the only
mention of that file (`work/issues/render-lanes-checkout-merge-ref-vanishes`)
calls it **lane-private**, so it was never committed. The enumeration
above is built from the tracker instead, and its method is stated so it
can be checked or widened.

**Q2.2 — the price of running all six points.** The shape matters more
than the count: **eps is read at runtime**, so the six points are **two
builds and twelve test jobs**, not six builds (ci.yml's *BUILD ONCE PER
COMPILE MODE*). And the test jobs are the cheap part. Measured medians
over the post-flip population:

| row | median | n |
|---|---|---|
| `build + archive (default)` | 309 s | 66 |
| `build + archive (interval)` | 369 s | 96 |
| a `test (…)` job, default lane | **46 s** | 128 |
| a `test (…)` job, interval lane | **58 s** | 186 |
| `clippy` | 66 s | 66 |
| `clippy + doc-tests (interval)` | 114 s | 96 |

A code-tier run today gates one point: one build, two test jobs, one
clippy row ≈ 344 + 106 + 94 = **544 job-seconds** (weighting the lanes by
their observed 59/41 split). Un-sampled it is two builds, twelve test
jobs and both clippy rows = 678 + 624 + 180 = **1482 job-seconds**.

* **+938 job-seconds ≈ +15.6 job-minutes per code-tier run**, taking the
  median run from **24.4 to ~40 job-minutes**.
* At **10.3 code-tier PR runs an hour** (149 runs over the 14.45 h
  window), that is **+161 job-minutes an hour ≈ +2.7 mean concurrent
  jobs**, against a measured mean of 4.3, p90 12, peak 36, and a queue
  delay of 3 s median / 25 s p99.
* **Wall clock: about +22 s on the median run, ~5 %.** Un-sampling does
  not lengthen the critical path, it makes the *slower* lane mandatory:
  runs that gate the interval lane already finish their last test row at
  a median **464 s** against the default lane's **385 s**, and 442 s is
  today's mixed median.

So it is roughly **3× F3's cost in runner load and a twentieth of it in
latency** — because what doubles is the build and what multiplies by six
is a 46–58 second job.

Not priced here, and named rather than absorbed: **`k-lint` is a second,
independent sampler** (1 of 5 unifications), and it is the one sampler
with a recorded red to its name (instance 4). Its job median is 127 s
with a p75 of 347 s and a max of 1039 s, so un-sampling *it* is a
materially different sum from the one above and wants its own reading.

**Q2.3 — is it independent of F3? In mechanism yes, in value no.**

* **Orthogonal in mechanism.** Sampling decides *which point* a run
  gates; F3 decides *which trees get a run at all*. Neither touches the
  other, and the evidence agrees: un-sampling would have caught neither
  composition (both point-independent), and restoring the push gate does
  nothing for a defect at an undrawn point unless that push run happens
  to draw it.
* **Complementary in value, and the direction is worth stating.** A
  restored push gate that samples gates the landed tree at **one of six**
  points; un-sampled, it gates it at all six. So un-sampling raises what
  option A is worth, and A raises what un-sampling is worth, but neither
  is a precondition for the other. **Ev's "regardless of if we still need
  the fix described in this pr" is supported.**

**Recommendation: un-sampling is its own unit**, filed as
`work/ciw/configuration-sampling-outlives-its-premise`. It is priced
independently, it changes no F3 decision, and its likely edit site is
`scripts/ci-filter.py` — **S-TCOST's territory**
(`work/tcost/program.md` `paths:`), not CIW's. CIW owns the posture
question and `.github/workflows/*`; any edit to the filter is S-TCOST's
to make or to be announced to them, and this unit writes neither.

## THE MERGE QUEUE, PRICED — and it dominates the push gate

Ev, in chat 2026-09-04: *"feel free to reinstate full runs instead of
sampling and also to experiment with a merge queue if that still ends up
looking important."* It does, and this section is the reason this unit
now recommends against its own opening proposal.

**Naming, because the letters have drifted.** Ev's comment reads `(A)`
and `(B)` as this document's options table — full job set versus test
rows only — and a later relettering would silently change what he
answered. So the options are **named** from here on: *push-gate (full)*,
*push-gate (tests only)*, *stop cancelling push runs* (a modifier on
either, not an alternative), *scheduled run*, *merge queue*, *nothing*.

### Why it is a different kind of answer

Every instance in this unit's evidence is a composition that **no run
ever compiled**: M10's tag values with LIB's gate, CHROME's
`blamed_mates` with M10-7's `Unleverable`. A push gate tests that state
**after it lands** — 5 m 48 s after, at best. A merge queue tests the
prospective merge **before** it lands, so `main` never carries the
defect. On both instances the queue does not detect faster; it
**prevents**.

And it closes the window at the source rather than compensating for it.
F3's residue is the gap between a PR's last run and its merge; a queue
*is* the elimination of that gap, because the tree that lands is the
tree that was tested. That makes the push gate largely redundant for
this class rather than complementary to it: **if the queue lands, the
push job set should not be restored** — they are alternatives.

### The throughput objection, checked and not sustained

The objection put to this lane was that a queue serialises merges, that
the median gap between pushes is 308 s against a 442 s run, and that a
serial queue would therefore build a backlog. **The median gap is not
the arrival rate.** Merges are bursty: p25 140 s, median 308 s, p75
874 s, p90 2489 s, and the **mean inter-arrival is 826 s** over the
45.66 h window. Mean service is 221 s (0.45 × 442 s code-tier + 0.55 ×
40 s docs-tier), so **utilisation is ρ ≈ 0.27**. A single-server queue at
ρ = 0.27 does not back up; it shows the burstiness as delay.

Replaying the **200 observed merge arrivals** through a FIFO server
(442 s for any group containing a code-tier merge, 40 s otherwise):

| batch size | ready-to-merge → landed, median | p75 | p90 | max |
|---|---|---|---|---|
| 1 (strictly serial) | **442 s** | 597 s | 903 s | **1770 s** |
| ≤2 | 442 s | 535 s | 807 s | 1541 s |
| ≤5 | 442 s | 463 s | **676 s** | **857 s** |
| ≤10 | 442 s | 463 s | 676 s | 857 s |

Batching past 5 buys nothing at this arrival pattern. **The cost of a
queue is therefore merge latency, and it is a median of 442 s with a p90
of 676 s and a worst observed burst of 857 s at batch ≤ 5** — against
~0 s today, since a lane merges the moment it is green.

**And it costs no more runner time than the push gate does.** Same
replay, counting the runs a queue would actually start:

| | code-tier runs/h | job-minutes/h |
|---|---|---|
| merge queue, batch 1 | 1.97 | **48** |
| merge queue, batch ≤5 | 1.80 | **44** |
| push-gate (full) | 1.97 | **48** |

Identical, because both run one full gate per merge; batching makes the
queue slightly cheaper.

**What the simulation assumes**, so it can be discounted properly: one
median service time per class rather than a distribution; the observed
arrivals were produced under a no-queue regime and lanes might merge
differently under one; and **failures are not modelled**. A red queue
run with batch N delays every PR in the batch and needs a bisect —
which is the argument for a small batch, and at batch 1 there is no
bisect at all. At the observed rate of composition defects (2 in
45.66 h) the bisect path is rare, but "rare" here is n = 2.

### What it would take, enumerated and NOT written

- **`merge_group` is absent from the tree**: it appears nowhere in
  `.github/` or `scripts/`. `ci.yml`'s `on:` needs the trigger.
- **The gate guards already do the right thing, and that is not luck.**
  Every gated job reads `github.event_name != 'push'`, so a
  `merge_group` event runs the full set with no edit. ci.yml's header
  argued for that spelling on 2026-08-28 — *"a trigger added here runs
  the gate by default and has to be excluded deliberately, which is the
  direction that fails safe"* — and this is the case it was written for.
- **`renders`' `push_to`** is `github.event_name == 'push' && …`, so it
  is empty under `merge_group`: report-only, correct by default.
- **`render.yml`'s concurrency group** keys on `github.ref`, which under
  `merge_group` is the queue's own `gh-readonly-queue/…` ref — distinct
  per group, so no cross-cancellation. Also correct by default.
- **Three `github.event.pull_request.base.sha` sites** (ci.yml:889,
  2506, 2971) are null under `merge_group`. The territory step at :889 is
  already `if: github.event_name == 'pull_request'`; the two test-cost
  report steps are `continue-on-error: true`, so they degrade rather than
  break — but they degrade silently, which is this repo's least-liked
  failure shape.
- **The change filter's basis** is the open question: it classifies a
  diff, and under `merge_group` the diff is the queue group against the
  base branch. Whether `scripts/ci-filter.py` computes that correctly is
  unverified here and is **S-TCOST's file**.
- **Required status checks vs sampled job names.** Branch protection
  requires checks *by name*, and the sampled matrix's names are computed
  (`test (eps = ${{ needs.filter.outputs.eps }}, 1/2)`). A queue's
  required-check list is hard to write against names that change per
  run — which is a second, independent argument for Q2's un-sampling,
  and the two changes fit together.
- **The working agreement changes.** `CLAUDE.md`: *"Agents own this
  codebase and merge their own PRs to main."* Under a queue an agent
  enqueues and waits, and a queue rejection hands the PR back. That is a
  culture change, not a workflow knob.
- **Blast radius**: it is a branch-protection setting; a misconfiguration
  blocks all merging for everyone. **Nothing here is enabled, and no
  repository setting was touched.**

### What the queue does NOT fix

- **Nothing about configuration sampling** (Q2). A queue run draws one
  point of six like any other run.
- **Instance 3's class**: a queue run classifies by tier like any other
  run, so rows a PR's tier skips are skipped in the queue too.
- **It does not make `main`'s push runs do anything more than they do
  today** — it makes them unnecessary for this class instead.

## THE OPTIONS, RE-PRICED

| option | runner cost | what it does to the two composition instances | what it costs elsewhere |
|---|---|---|---|
| **merge queue** (batch ≤5) | **44 job-min/h** | **prevents both** — they never reach `main` | merge latency: median 442 s, p90 676 s, max 857 s; a branch-protection setting; the merge culture changes |
| **push-gate (full)** + stop cancelling | 48 job-min/h | detects both, 5 m 48 s after landing | a three-mechanism concurrency design pass |
| **push-gate (full)**, cancellation as-is | 48 job-min/h | detects **1 of 2**, by a 6–27 s margin | — |
| **push-gate (tests only)** + stop cancelling | 16 job-min/h | detects both, the second at +127 s not +84 s | same design pass |
| **push-gate (tests only)**, cancellation as-is | 16 job-min/h | detects **0 of 2** | — |
| **scheduled run** | 9.8 job-h/day hourly | detects, ≤1 h later, naming ~4.4 merges | — |
| **nothing** | 0 | 17 m 29 s, on a stranger's branch | 42 red runs on 20 branches, and 34 m 25 s of non-compiling `main`, in the two observed instances |

## THE RECOMMENDATION, REVISED

**Design and trial the merge queue; do not restore the push job set
unless the queue is rejected.** This unit was opened to re-cost F3 and
the honest answer is that the re-costing found a better instrument than
the one it was asked to price: at **the same runner cost** (44 vs 48
job-minutes an hour) the queue **prevents** the defect class that the
push gate can only **detect**, and it removes F3's residue at its source
rather than compensating for it.

**The price to weigh is merge latency, not minutes**: median **442 s**
from ready-to-merge to landed, p90 **676 s**, worst observed burst
**857 s**, against approximately zero today. That is the number Ev
should be answering, and it is a working-rhythm question rather than a
CI one.

**If the queue is rejected, the fallback is push-gate (full) plus the
concurrency design pass, at 48 job-minutes an hour** — and *not* the
test-rows-only variant, which the historical evidence now rules out: it
catches 0 of 2 as things stand and 2 of 2 only with the same design
pass, always later than the full set, because the row that catches
instance 2 fastest is `clippy`.

**F3 itself needs no revision under either answer**, which is worth
saying plainly after all this measurement: a queue makes the push run's
job set irrelevant to correctness, and the push run keeps carrying its
three write side-effects exactly as F3 left it.

## Ev's ruling (2026-09-04): trial the merge queue

Asked in chat and answered there: **trial a merge queue.** That is option
D, this unit's own recommendation, and it is the option that *prevents*
the composition defects rather than detecting them — at 44 job-min/h at
batch ≤5 against the push gate's 48, both $0.

Consequences recorded here so the next reader does not re-derive them:

- **The push gate is not restored.** Options A (full job set on
  `push: main`) and B (the per-SHA concurrency design pass) stay
  unbuilt, and are the fallback if the queue trial is rejected. If they
  are ever taken it is the FULL set plus B, never the narrow variant —
  the narrow variant catches 0 of the 2 recorded instances because the
  row that catches instance 2 is `clippy`, which it does not restore.
- **F3 itself needs no revision either way**, as measured above.
- **The scheduled run stays declined** (Ev, 2026-08-22), unchanged by
  this ruling.
- **Un-sampling was the precondition and it has landed** — a queue's
  required checks are named, so the job set had to stop moving with a
  seed first. `work/ciw/reinstate-full-configuration-runs` (PR 1823).

The trial itself is a separate unit: `merge_group` needs no `ci.yml`
edit (the `!= 'push'` spelling already admits it), but the required-check
list, the batch size, the bisect behaviour on a failed batch and the
branch-protection setting are a design pass, and enabling it changes how
every agent in this repository merges.

## Folded in from PR 1805 (code-quality Track T), 2026-09-04

That PR asked the same question ~90 minutes before this one and is closed
as superseded, with two things carried across rather than lost:

**An incident this unit's enumeration missed, and it lands on the one
dimension still sampled.** `#1756 → #1775`: `k-lint (gate)` reported
**green with `demos tour fmt + clippy` skipped**, because the drawn row
did not carry it. This unit's Q2 answer says "zero of five recorded
main-reds are attributable to the draw" — true of the **lane/ε** draw
that PR 1823 removed, and **false of the k-lint draw**, which is still
one-in-five. So `work/ciw/klint-row-still-sampled` is no longer a
cost-shape deferral with no known cost: it has a measured instance of the
remaining sampled dimension hiding a real failure. That raises its
priority and the item says so.

**Overtaken the same day (2026-09-04, PR 1850).** The k-lint draw is
gone: five matrix legs, no draw, `KLINT_ROW=all` on every run. So "which
is still one-in-five" above is a record of what was true when written,
and this unit's Q2 answer — "zero of five recorded main-reds are
attributable to the draw" — no longer has a surviving draw to be scoped
against. `klint-row-still-sampled` is closed by that PR.

**A correction of the shape this program kept making.** PR 1805's author
first filed the two-green-PRs class as a *finding*, then withdrew it on
discovering `ci.yml` already documents it in the F3 note — *"a semantic
conflict between two independently-green PRs surfaces at the NEXT PR's
merge-ref rather than at the merge that caused it… THE COST THAT REMAINS,
stated rather than mitigated: the conflict surfaces on an INNOCENT PR."*
Re-filing a ratified trade-off as a discovery is a real failure mode and
the withdrawal is the right handling. `work/issues/two-green-prs-merge-into-a-red-main.md`
is deleted here, as that PR intended.

**What is NOT withdrawn** is
`work/ciw/merge-order-semantic-break-reaches-main`: that records a dated
*instance* (the `MateFault::Unleverable` break, main non-compiling for
34 m 25 s) as evidence for this unit, not the class as a discovery. An
instance of a ratified residue is worth keeping; the class was already
written down.

## Three verdicts here are superseded by the trial design (2026-09-04)

`work/ciw/merge-queue-trial` re-derived this unit's queue pricing after
PR 1823's un-sampling landed, and **three** things in the sections above
do not survive it. Corrected here rather than left for the next reader.
(Two corrections and a closure when this was written; the third
correction — `renders` — was added on 2026-09-04 after a style review
pointed out that the unit reversed that verdict and this addendum left it
standing.)

1. **The batch-size table and the "44 job-min/h at batch ≤5" figure are
   wrong in MECHANISM, not only in magnitude.** They come from a
   simulation in which batching reduces the number of CI runs. GitHub's
   merge queue builds one merge group **per queued pull request**, and
   its documentation says so directly: *"Merge limits do not combine
   `merge_group` **builds**. Merge limits only affect merges to the base
   branch once one or more `merge_group` has satisfied build checks."*
   So a queue costs one full gate per pull request at any batch size,
   the "batching makes the queue slightly cheaper" line is void, and the
   lever that actually moves latency is **build concurrency**, which
   this document does not name. Re-measured after un-sampling, the gate
   is 44.8 job-minutes rather than 24.4 — and a MERGE GROUP does not run
   the render lanes (see 3 below), which are 330 of those 2686
   job-seconds, so a queue run is **40.2 job-minutes** and the queue costs
   **99 job-min/h**, not 44. *The conclusion is unchanged*: the queue
   still prevents where the push gate detects, and it still needs no
   concurrency design pass, which the push gate does. (This addendum first
   said 110 job-min/h, on the pull-request run cost; corrected here on
   2026-09-04 after a style review caught that ~13 job-min/h of it was
   render work no merge group runs.)
2. **"The two test-cost report steps degrade silently" is false.**
   `scripts/base-test-listing.sh:86-88` answers an empty `BASE_SHA` with
   a stated skip — *"this run is not a pull_request run, so it has no
   base tree to diff against"* — into the job log and the step summary.

3. **`renders`' "correct by default" verdict is REVERSED, not merely
   qualified.** This document's enumeration reads *"**`renders`' `push_to`**
   is `github.event_name == 'push' && …`, so it is empty under
   `merge_group`: report-only, **correct by default**"*. The `push_to`
   half of that is true and the verdict is not: the job's own guard is
   `!= 'workflow_dispatch'`, so under `merge_group` the whole reusable
   workflow **would have run** — rendering a tree the pull-request run has
   already rendered and `main`'s push run re-renders, against a
   `gh-readonly-queue/…` sha that is deleted at the merge.
   `work/ciw/merge-queue-trial`'s PR excludes it from `merge_group`
   outright. "Correct by default" would have left a reader of this
   document with the superseded answer.

One thing this document left open is **closed** rather than corrected:
the change filter's basis under `merge_group` ("unverified here") is
right with no edit, because a merge group's first parent is the group
before it, so `git rev-parse HEAD^1` yields exactly the one pull request
that group adds.
