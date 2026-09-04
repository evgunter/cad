---
id: f3-recosting-on-a-public-repo
kind: unit
title: F3 and the nightly demotions rest on an Actions allowance this repo no longer has: re-cost them on a public repo
status: open
opened: 2026-09-04
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
names the run or job it came from. The population, wherever one is
quoted, is **completed `pull_request` runs of `ci.yml` created after the
visibility flip** — 220 runs between `2026-09-03T15:20Z` and
`2026-09-04T05:47Z`, of which **149 are code-tier** (their `build +
archive` job is not skipped). Nothing from `docs/CI-MINUTES-2026-08.md`
is quoted forward; where a figure of its is named it is named as the
thing being replaced.

### M1 — the runner, read first-hand rather than taken from prose

Run **33830873453**, job **100893490483**, step `runner + linker
provenance (for perf attribution)`: `nproc` prints **4** and `free -g`
prints **15** GB total. `483212ef`'s "4 vCPU / 16 GB" is confirmed at
the runner, not just at the comment. The same job's rust-cache step
prints `Cache hit for: v0-rust-build-default-Linux-x64-fa41882e-fd5fb1c1`
… `full match: true`, 263 MB — see the last section.

### M2 — what a full code-tier run costs on that runner

| figure | value | n |
|---|---|---|
| run created → last job end | **442 s (7.37 min)** median; p25 390, p75 480, max 1086 | 149 |
| first job start → last job end | 437 s median | 149 |
| the same, **`tier=all` runs only** (`--workspace`; the `interval backend crate` job ran) | **476 s (7.9 min)** median, max 1066 | 66 |
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
github.ref }}` (ci.yml:103–105) therefore cancels most of them. It
already does, at today's 40-second push run: **67 of 200 (34 %) are
`cancelled`**.

**This is not a hypothetical, and it lands on the exact evidence this
item rests on.** The push run for LIB's merge — the commit that composed
the defect — is run **33787453014**, started `17:55:42Z`, **cancelled at
`17:59:48Z`** (246 s) because the next push's run **33787719180**
started at `17:58:23Z`. A restored full run needs ~442 s to a verdict
and ~350 s to reach the first test row. **Restoring the job set alone
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
  the tests hang off it. F4's "build is ~88 % of it" survives the runner
  change as a shape even though its number does not.

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
after the merge that caused it. F3's control did not fail and it was not
slow; it named a stranger. The 8 h 11 m is the distance from a red
nobody owned to a repair, and the 42 runs on 20 branches is what was
billed to lanes that had written none of it.

That is the cost to weigh: **not detection latency — attribution.**

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
| **A. full job set on `push: main`, + per-SHA concurrency for push runs** | **+48 job-min/h (+0.80 mean concurrent jobs); $0** | 442–476 s after the merge | the merge commit, hence both PRs |
| **B. as A but test rows only** | +16 job-min/h (+0.26); $0 | 433 s | same |
| **C. scheduled full run** | 9.8 job-h/day hourly; 24.4 job-min/day nightly | ≤1 h / ≤24 h | a window of ~4.4 merges (hourly) |
| **D. do nothing** | 0 | 11 m 41 s, measured | a stranger's branch; 42 red runs on 20 branches in the one observed instance |
| **A or B without the concurrency change** | as above | never, on 64 % of merges | — |

## RECOMMENDATION

**Option A.** Restore the full job set on `push: main` — that is,
revert F3's `github.event_name != 'push'` guards — **and give push runs
a concurrency group of their own keyed on `github.sha`, so a merge's
gate is not cancelled by the next merge.** The two halves are one
proposal; A without the second half is measurably worthless (M3).

**The number: +48 job-minutes an hour of runner time, +0.80 mean
concurrent jobs against a measured mean of 4.3 and a queue delay of
3 seconds, and $0.** A verdict lands ~7.4 minutes after each merge, on
the merge commit.

**Why the full set rather than B.** B is a third of the load, and the
load is not the binding constraint — nothing queues today, and the
difference between +0.80 and +0.26 concurrent jobs is not a difference
anyone can observe. What B gives up is every composition that is not a
test failure, and this unit has exactly **one** observed instance to
generalise from. Buying the coverage at a cost nobody can feel is the
better trade; B is the answer if the footprint matters more than the
coverage, and it is one line different.

**What A does NOT buy, stated plainly.** It does not remove the 42 red
runs. PR runs still red on a broken `main`, and they still red first —
11 m 41 s against ~7.4 min is a four-minute head start, not a rescue.
What A buys is that the red exists **somewhere that names its author**
while those PR runs are going red. Whether that shortens the 8 h 11 m
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
on 2026-09-03 and write under the default branch's scope, and job
**100893490483** on a PR branch restored
`v0-rust-build-default-Linux-x64-fa41882e-fd5fb1c1`, 263 MB, `full
match: true`. That is the "first reading owed" which
`docs/CI-MINUTES-2026-08.md`'s cache section asks for, and it says the
primer works. S-TCOST's item to update; cited here, not edited.
