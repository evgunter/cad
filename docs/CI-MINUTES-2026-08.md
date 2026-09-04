# CI minutes audit, 2026-08-20

Why: the Actions allowance was being consumed faster than the work
justified. This is the measurement, not a plan — the two changes it
licensed are listed at the end, and the two it did NOT license are
listed with them.

## Method, and the billing model that makes it non-obvious

GitHub bills **per job, rounded up to the nearest minute**, and the
`get_workflow_run_usage` endpoint returns zeros for this repo, so the
numbers below are computed from each job's `started_at`/`completed_at`
via the jobs API. Note that endpoint **paginates at 30**: the first
reading of the reference run below said 30 jobs / 80 minutes and was
wrong. It has 37 jobs.

The round-up is the part that does not show up in wall-clock
intuition. A job that runs for four seconds costs the same minute as
one that runs for fifty.

Reference runs:

* **code-tier PR** — run `32425890937` (PR #823, `smelle/s14`)
* **docs-tier push** — run `32424711536` (main, `de6a0c7`)

## The code-tier run: 37 jobs, ~64 min wall, ~87 billed

| job | wall | billed |
|---|---|---|
| build + archive (interval) | 11.90 | 12 |
| build + archive (default) | 11.03 | 12 |
| k-lint (gate) | 9.78 | 10 |
| render lanes (5 legs) | 8.82 | 12 |
| test, default build (6 legs) | 6.27 | 8 |
| test, interval build (4 legs) | ~1.1 | 4 |
| clippy + doc-tests (interval) | 2.30 | 3 |
| clippy | 1.72 | 2 |
| python suite | 1.58 | 2 |
| rebuild latency (reporting) | 1.57 | 2 |
| corrupt input (release profile) | 1.37 | 2 |
| persistence (2 eps legs) | 2.61 | 4 |
| band 4 corpus (2 eps legs) | 2.45 | 4 |
| watertight, step import, rustfmt, rustdoc, discipline, filter, docs-only, cleanup | 3.52 | 8 |
| interval oracle, interval backend | skipped | 0 |
| **total** | **~64** | **~87** |

**Critical path is 13.75 min and is essentially one job.**
`build + archive (interval)` runs 0.6 → 12.4; the longest test leg
then finishes at 13.6. Build is ~88% of it. Nothing outside the two
build jobs and `k-lint` can move the run's latency much.

**~23 minutes of the 87 are the per-job round-up** (87 billed vs ~64
wall). That is the single largest "cost with no work behind it", but
it is spread thin: most jobs are 1–3 minutes, where merging either
crosses no minute boundary or trades against latency.

## The docs tier is already solved

Run `32424711536`: **21 of 23 jobs skipped, ~2 billed minutes.**
`scripts/ci-filter.py` is doing its job. No further work is warranted
on the docs path; the burn is entirely code-tier runs.

## Volume

Runs 2211–2240 span 25 minutes; runs 2421–2450 span 38 minutes —
roughly **60 runs/hour** during active work, of which (by duration)
about **13 in 30 are code-tier**. Concurrency cancellation is already
enabled and working (`cancel-in-progress`, ci.yml:9); a large fraction
of runs are superseded within a minute or two and cost little.

## Findings

### F1 — render lanes on PRs: NOT waste. Do not gate them off.

Rejected after being briefly landed and reverted (commits `d2e8301`,
`e5f4a02`). The lanes cost 12 billed minutes per code-tier PR run and
cannot re-baseline off main (`push_to` is empty on `pull_request` by
design, ci.yml:2023), which reads as pure waste. **It is not.**

`local-scripts/render-hosted.sh` (its header, and the take path at
:163–193) makes taking CI's artifacts **the default and dispatching
the flag**: with no arguments it runs `gh run list --workflow ci.yml
--branch <branch>` and downloads each lane's artifact from that run.
Gating `renders` to pushes empties that run of lanes and artifacts,
and the script deliberately refuses to fall back to a dispatch
("silently taking the expensive one is the kind of helpfulness that
surprises"). So the 12 minutes are not a report nobody reads — they
are the agent-facing render path.

A real reduction here has to change **both halves together**: gate the
lanes off PR runs *and* make `render-hosted.sh`'s default dispatch on
demand (~5 runner-minutes, only when someone actually wants frames,
versus 12 on every code-tier run). That is a change to a documented
agent tool, not a CI tuning knob, and belongs to Ev.

### F2 — the round-up, where it is collectable

Two merges land (below), worth **4 billed minutes** at **zero**
latency cost — both merged jobs sit far off the critical path.

**Explicitly rejected: merging the default-build test shards.** The
legs are ~10 s setup + 37–63 s execution. Merging `default 1/2` (63 s)
and `2/2` (56 s) gives ~129 s against 73 s today: **+0.93 min latency
(+7% of the whole run) to save 1 billed minute.** The shards are
already sub-minute, so the round-up is not biting there.

Not yet done, same pattern, ~3 more minutes, each independent:
`persistence` (2 eps legs → 1), `corpus` (2 eps legs → 1), and
possibly `clippy` + `clippy (interval)`.

### F3 — post-merge runs on main duplicate the PR run — LANDED

Every merged code PR pays ~87 minutes a second time. Only **two**
things are unique to a `push` run, and both are *write* side-effects
rather than gates:

1. `rebuild-latency`'s `commit measurement to the timing history
   (main only)` step (ci.yml:1583), appending to
   `docs/perf-data/rebuild-latency/`.
2. `renders`' `push_to`, which re-baselines render cells
   (ci.yml:2023).

Neither can simply move into PR runs, and ci.yml:2014 records the
measurement that says why: a `GITHUB_TOKEN` bot commit onto a PR
branch becomes the PR's head, triggers no run of its own, and strands
every green check on the parent — observed on #598 as "30 green jobs
on 048edc9, and a head commit carrying a single check".

**Landed** (Ev authorised, 2026-08-20; commit `0768882`): keep the `push: main` trigger but
reduce it to `filter` + `rebuild-latency` + `renders`, skipping build,
test, clippy and k-lint. Preserves both write paths; drops ~65 of the
87 minutes per merged code PR.

**What it gives up:** the landed main commit is then never itself
tested. PR runs test the merge-ref, so when main moves between a PR's
last run and its merge — frequent at this repo's merge rate — that
exact combination went untested.

**The scheduled full run on main that would have paired with this is
DECLINED** (Ev, 2026-08-22). The next PR's merge-ref is main plus
that branch, so it tests the landed tree anyway; a scheduled run buys
a second discovery of the same fact and costs a full gate per period
whether or not anything landed. The residue is accepted, not
outstanding: **a semantic conflict between two independently-green PRs
surfaces on the next, innocent PR** rather than at the merge that
caused it, and the person who gets the red did not write the code that
caused it. The reading that goes with that is in ci.yml's header.

### F4 — sccache: measured, and it does not pay — OFF BY DEFAULT

`docs/LOCAL-BUILD-PERF.md`'s sccache section reverted sccache locally on
a real measurement: cold build 156 s → 96 s at a 99.4% hit rate, given
up because sccache and incremental compilation are mutually exclusive
and going non-incremental cost **5–7x on the edit-rebuild loop** (91 s
vs 18 s on geom-core). `local-scripts/gate.sh` keeps sccache on the
local gate runner for exactly the cold-build case.

**That objection is moot on the runner** — `Swatinem/rust-cache` sets
`CARGO_INCREMENTAL=0` itself on every job that uses it — which is what
made the question worth asking here at all. The rig landed in #852
behind the repo variable `SCCACHE`, with the reading owed in a few
days' time. **The variable was then set to `"0"` and the reading was
never taken**: on every run from then until 2026-09-03, both build jobs
reported `install sccache` and `sccache stats` as `skipped` (e.g. runs
33719350040 and 33718880979). TCOST-C4 dropped the condition on a
branch — a branch cannot change a repo variable — and took it.

**THE READING** (PR 1648, `tcost/c4-sccache-reread`; every run at
`tier=all`, the same 18-package set, lane asked for by trailer, so the
rows are like-for-like in B1's sense):

| run | head | lane | rust-cache | sccache object cache | `build … + archive` | sccache stats |
|---|---|---|---|---|---|---|
| 33721067389 | af3d51bf | default | cold | nothing to restore | **799 s** | 302 requests, **0 hits**, 191 misses, 104 non-cacheable (90 `crate-type`) |
| 33722975323 | 2eb2cf45 | interval | cold | nothing to restore | **910 s** | 302 / 0 / 191 / 104 (90) |
| 33724962116 | 01b90e47 | default | cold | **miss** at a 38-minute gap | **787 s** | 302 / 0 / 191 / 104 (90) |
| 33726782739 | eaead9ab | default | **restored** | **hit** at a 9-minute gap | **534 s** | 68 requests, **18 hits**, 0 misses, 50 non-cacheable (47 `crate-type`) |
| 33729282948 | 9862ccc0 | interval | cold | **miss** at a 60-minute gap | **860 s** | 302 / 0 / 191 / 104 (90) |
| 33738656130 | 582f50d3 | interval | cold | **miss** at an 88-minute gap | **854 s** | 302 / 0 / 191 / 104 (90) |
| 33741629684 | 5a16dddd | interval | **restored** | **hit** at a 17-minute gap | **661 s** | 68 requests, **5 hits**, 13 misses, 50 non-cacheable (47 `crate-type`) |

The control is a run of the same shape with the rig inert: run
33719350040 (`smell/k-lint-readings`, head 70182ddf), default lane,
`tier=all`, same package set — **769 s**. Its rust-cache reported `No
cache found` on `v0-rust-build-Linux-x64-6f07d2f1-66da18f8` and its post
step then saved 275 901 512 bytes under it; the trial branch's key
differs from that one in exactly the env-hash component, which is what
`RUSTC_WRAPPER` moves. B1's twelve `tier=all` interval samples
(570–869 s, PR 1616) are the interval control population; the spread is
±25 % and every number above sits inside it. Every row here, the control
included, has its own file under `docs/perf-data/sccache-trial/`,
carrying the rust-cache key it restored or missed.

**WHAT THE WARM RUNS SAY.** The two runs where both caches restored are
the whole trial. Each had exactly **18 cacheable units and 47
refusals**:

* the 18 cacheable units are the workspace **libs** — every rlib cargo
  still had to build once rust-cache had served the dependencies;
* the 47 `crate-type` refusals are the **binaries**: the test binaries
  in the archive and the workspace build scripts.

The default lane took all 18 of its cacheable units from the cache; the
interval lane took 5 and rebuilt 13, because a workspace rlib that is
not bit-identical between runs changes the key of everything downstream
of it. That difference is worth exactly what the 18 are worth, which is
the point: **sccache 0.16.0 does not cache `--crate-type bin`**, and a
test binary is a bin. Verified directly on the pinned binary: a
two-target crate gives one cacheable rlib and one `crate-type` refusal
for the bin, and
`cargo test --no-run` on it gives **three requests, three refusals, all
`crate-type`**. The cold runs' `crate-type 90` is the same set plus the
dependency graph's own binaries: 47 of the 90 are the workspace's, and
the remaining 43 are dependency build scripts, which cargo compiles as
bins and which a warm run never asks for at all.

So the hypothesis this rig was built to test cannot be true. The split
inside the workspace's compile time, from the build-side census taken
2026-09-03 on a 4-core lane box under CI's own profile env
(`OPT_LEVEL=1`, `DEBUG=line-tables-only`, `TEST_STRIP=debuginfo`,
`CARGO_INCREMENTAL=0`), over 297 units and 1 178 unit-seconds:

| workspace bucket | unit-seconds | share |
|---|---|---|
| test targets (`all` binaries 535 s, `--lib` test binaries 191 s) | **726.3** | **82 %** |
| libs | **159.5** | **18 %** |

(The same census puts 259.6 s in external dependency libs and 32.6 s in
dependency build scripts, and a second contended run reproduces the
82/18 split at 78/19.) #853 called the test binaries "the whole
hypothesis". They are the exact set sccache will not touch, at any hit
rate, warm or cold. Its ceiling here is the 18 % — and only the part of
that 18 % a restored object cache covers.

**AND THE OBJECT CACHE BARELY PERSISTS.** Five restore attempts:
**hits at 9 and 17 minutes, misses at 38, 60 and 88** — the entry is
~205 MB per lane and this repo's 10 GB Actions cache budget churns it
out within the hour (on the 88-minute attempt even the 9 MB
`sccache-bin-…` entry had gone). #853's own item 3 named this as the
thing that would mean the trial measured nothing; a warm reading here
had to be manufactured by pushing again within minutes, which is not how
the gate is used.

**A THIRD FINDING, LARGER THAN THE ONE WE WENT LOOKING FOR.** The same
logs say `Swatinem/rust-cache` reported **`No cache found`** on five of
these seven build jobs *and on the control run* — each time compiling
all ~300 units. What that establishes, stated no wider than the
evidence: **a branch's first build job restores nothing, and a later one
restores only what that same branch saved, if the budget still holds
it.** (Seven of the eight jobs are one PR branch, and the two restores
were its own saves.) The cause is not in doubt: GitHub scopes a cache to
its branch plus the default branch, `push` runs to main do not run this
job (F3), so main holds no entry any PR could inherit — and the ~275 MB
entries a run does save are subject to the same eviction as sccache's
own. The premise under F4 and #853 ("rust-cache already caches the ~225
dependency crates, so sccache's unique contribution is the workspace
crates") does not hold for a first run on a branch, which is what most
gate runs are. **That is the lever worth pulling next**, and it is not sccache — it was
pulled on 2026-09-03 and the entry at the foot of this file carries the
keys, the 82 % miss rate and the priming jobs that answer it:
the 799 s → 534 s and 854 s → 661 s pairs in the table are what a warm
run looks like, and at most 18 of a warm run's 68 units came from
sccache.

**DISPOSITION.** The rig stays in the tree and is **off by default**:
the condition is now `vars.SCCACHE == '1'`, so it is inert with no
variable set and the repo variable `SCCACHE=0` can be deleted by
whoever holds the settings. It is not deleted, because the two steps
cost ~0 s while skipped and this note plus
`docs/perf-data/sccache-trial/` are what prevent a third pass at the
same idea. **Do not re-open this without a reason that answers the
`crate-type` refusal** — sccache caching Rust binaries, or a build job
whose cost has moved out of the test targets.

**A limit of the method, stated plainly.** No hosted A/B of sccache
alone is possible with this rig: `RUSTC_WRAPPER` is `RUST*`-prefixed, so
rust-cache hashes it, and turning sccache off rotates rust-cache's key
and buys a cold rebuild in the off arm. The two are measured together or
not at all; what separates them here is the stats step, not a pair of
durations.

### F5 — volume

Skipping full CI on draft PRs (`github.event.pull_request.draft`)
would let agents push intermediate commits freely and is the largest
single lever available. It is a workflow-policy decision, not a tuning
one, and is recorded here without a recommendation.

### F6 — the rustdoc gate's cache: the six excluded roots — LANDED

The lever named three times above ("still unmeasured") and ranked next
at #2. It is measured now, and it pays.

**The mechanism.** `Swatinem/rust-cache` caches the target directories
it is *told* about, and its default is one: `./target`. Since D40/D41
the rustdoc gate documents SEVEN cargo roots, and the six the workspace
excludes each build into a target directory of their own — `demos/tour/
target`, `demos/wild/target`, three under `tools/`, and
`interval-transcendentals/target`. No cache carried any of them, so the
`fmt` job rebuilt all six from nothing on every run, `demos/tour` and
`demos/wild` compiling the whole kernel each time, while the workspace
pass ran against a warm dependency graph.

**Measured on the runner** (ubuntu-latest, 2 vCPU; two jobs one input
apart, seeded on run `32575515632` and read warm on run `32583370980`
with one kernel doc comment changed — the case a real PR is):

| | today | told about all seven |
|---|---|---|
| cache restore | 14 s | 18 s |
| `doc-gate.sh --selftest` | 32 s | 28 s |
| `doc-gate.sh` | **67 s** | **33 s** |
| whole job | **136 s** | **99 s** |
| billed | 3 | **2** |
| cache entry, compressed | 155 MB | 245 MB |

**−1 billed minute per code-tier PR run**, which gives back one of the
two that D40/D41's widening cost. The +90 MB is paid only when the key
rotates — a warm run logs `Cache up-to-date` and re-saves nothing — and
the restore costs ~4 s more. On a COLD run the candidate is slightly
*slower* (gate 105 s against 91 s, job 172 s against 152 s): it has six
more target directories to compress and upload. That is the one-cold-run
tax every cache key rotation here pays, not a standing cost.

**Where the time is, locally** (4 vCPU, `./target` holding dependency
artifacts only and the six roots absent, i.e. the hosted starting state):

| pass | today | all seven cached |
|---|---|---|
| workspace, `--all-features` | 16.8 s | 16.7 s |
| `demos/tour` | 12.2 s | 3.5 s |
| `demos/wild` | 11.9 s | 3.4 s |
| `tools/tess-meter` | 6.5 s | 1.9 s |
| `tools/k-lint`, `tools/tess-lint` | 2.5 s | 2.3 s |
| `interval-transcendentals` | 1.4 s | 0.5 s |
| **total** | **51.3 s** | **28.4 s** |

The workspace pass does not move, and that is expected rather than a
disappointment: rust-cache evicts workspace-member artifacts before
saving (it keeps packages whose manifest lies *outside* the workspace
root), so the kernel crates rebuild either way. **The whole win is the
six roots**, and the two demo roots are two thirds of it.

**Two variants measured and rejected.**

* **`CARGO_TARGET_DIR` pointing every root at `./target`** — the other
  lever named above, and it is a dud: 51.3 s → 46.7 s locally, because
  `demos/tour`'s kernel units are built under its OWN feature selection
  and so carry different fingerprints from the workspace pass's. It
  rebuilds them anyway, in a shared directory instead of its own.
* **Also caching `target/doc`** — 29.1 s against 28.4 s, i.e. nothing.
  A crate whose fingerprint changed has its docs regenerated regardless,
  and rust-cache deletes the doc tree's contents before saving anyway
  (its cleanup recurses into `target/doc` and removes every file).

**What this does NOT touch, and it is now the larger half.** The
`--selftest` is ~30 s hosted and no cache reaches it: every case plants
a fresh fixture under `mktemp -d` and cargo keys its fingerprints on the
package's path, so each of the ~15 cases pays ~3 cold `cargo doc` runs
on dependency-free crates (measured: 0.5 s each cold, 0.02 s warm). A
shared target directory across cases would need a stable fixture path,
which would trade the harness's per-case isolation for seconds — the
wrong trade in the one gate whose subject is not being silently green.
If that 30 s is ever worth collecting, the lever is running the cases in
parallel, not caching them.

**The same hole is open in `k-lint (gate)`, and is worth more.** That
job (10 billed minutes, the third-largest line item) builds `demos/tour`,
`demos/wild` and all three `tools/` crates too — `cargo fmt`, `cargo
clippy --all-targets`, `cargo test`, and a `--release` eps pin — each in
the same uncached target directory, behind a plain
`Swatinem/rust-cache@v2`. It is the identical one-input fix. It is NOT
landed here: this finding measured the `fmt` job, and a claim about a
10-minute job wants its own reading rather than this one's, extrapolated.

> **SUPERSEDED for `k-lint`, 2026-08-22 — and the reasoning above is kept
> because it is still true, just no longer the lever.** The paragraph
> reads the 10 minutes as a CACHING problem. It is not one. Those rows
> are FIVE FEATURE UNIFICATIONS — default-features dev, default-features
> `--release`, `--release --features budget`, dev `--features budget`,
> dev `--features probe` — and they share almost no artifacts by
> construction: `--release` and dev are different profiles, and `budget`
> and `probe` are opt-in features gated at a module boundary, so every
> crate that sees one carries a different fingerprint under it. **No
> cache configuration collapses five distinct compilations into one.**
> What the `workspaces:` input would have bought here is the same thing
> it bought the `fmt` job — the excluded roots' dependency artifacts
> surviving between runs — against a job whose cost is five compiles of
> the kernel, not one cold one.
>
> What collapsed it instead is **sampling**: the job now draws ONE of the
> five per run, seeded independently of the lane and ε draws
> (`scripts/ci-filter.py`'s `KLINT_ROWS`), which takes it from ~10 billed
> minutes to ~2-3. The cache lever is not thereby wrong; it is
> proportionally much smaller, because it now applies to whichever single
> unification a run drew. The one piece of it that DID land is the cheap
> half: the job's `rust-cache` key now carries the drawn row's PROFILE, so
> the two dev draws and the two release draws each keep a cache lane
> instead of one entry thrashing between profiles.
>
> **Why sampling is admissible here and was not proposed for the rustdoc
> gate.** The rule is at *What is NOT sampled* below: sampling covers a
> detector whose subject PERSISTS in the tree. All five rows were audited
> one at a time against it before the wiring was written — a clippy
> finding, a failed ε pin, a grown triangle budget, a probe suite that
> stopped compiling all stay broken until someone fixes them, so a later
> draw finds them; and the one row that looked like an absence-detector,
> the probe census, is not one, because the half that would notice a probe
> suite *disappearing* (`probe-suite-census.sh` in its default mode, with
> its `CENSUS_FLOOR`) is sited in `discipline`, which is unconditional and
> unsampled. The rustdoc gate's six roots are *independent*, which is the
> different reason given below for not sampling that one: sampling buys
> latency there proportionally rather than exploiting near-certain
> agreement.
>
> **What it costs, recorded because it is a real cost and nothing else
> records it.** Two ratified review outcomes read on those rows as
> UNCONDITIONAL and are now 1-in-5: MIN-1's per-triangle certificate
> falsifier (the `dev-budget` row), and `crates/sweep/tests/k_report.rs`
> plus `docs/K-REPORT.md`, which both say the probe harness is
> type-checked and run *"on every building merge"*. No gate reds on
> either — the census gate greps for the step NAME, not for how often it
> runs — so those two sentences are simply false in that one phrase and
> are owed a correction. Said out loud here and at the job, which is what
> ci.yml's own step comment asks for.
>
> **The correction is written (2026-08-30), at all three sites.**
> `crates/sweep/tests/k_report.rs` says *"1 in 5"* and names the
> `dev-probe` row it rides. `docs/K-REPORT.md` names the row and its
> schedule at each of the four sentences that carried a frequency claim,
> and marks the one that is genuinely unconditional
> (`probe-suite-census.sh`'s default mode, sited in `discipline`) as such
> rather than demoting it with the rest. And MIN-1's falsifier: its own
> step comment in `ci.yml` asserted *"THE INTENT SURVIVES ONLY WHILE THIS
> ROW STAYS UNCONDITIONAL … the falsifier still runs on every
> build-triggering change"* — three lines above its own
> `if: contains(fromJSON('["dev-budget", "all"]'))`. It now states the
> 1-in-5 schedule and says that no path pin restores it, since
> `crates/mesh` is not a pinned root. **The first pass of this very
> correction missed that site**, having taken the debt's two K-REPORT line
> numbers as the work; that is the discharge-by-line-number class the
> correction is about, recurring inside it. **The debt line stays here** —
> a minutes entry is a record of what a decision cost, and deleting the
> cost once it is paid leaves the decision looking free.
>
> **And the schedule improved under it rather than only being described.**
> A change under `tools/` now PINS the k-lint row that compiles it instead
> of drawing one (`KLINT_PATH_ROWS` in `scripts/ci-filter.py`, Ev's
> ruling of 2026-08-29). That is *unconditional-when-`tools/`-changes*,
> which is a real schedule and is what the corrected sentences point at.
> It does not restore any of the three claims above to *unconditional*:
> `k_report.rs` and the falsifier live under `crates/`, which the pin does
> not reach.

**ADDENDUM, 2026-08-31 — F6's result may have been spent, and this
entry says so rather than leaving the −1 standing unqualified.** This
document maintains its billed-minute figures BY HAND, deliberately and
for the reason given under *Method*: they are not guardable, so the only
thing that keeps them honest is a change that moves one saying so at the
entry it moves. The D180/D301 widening (the rustdoc gate's `not(feature)`
blind spot) has added a **third pass** to `scripts/doc-gate.sh` — one
`--no-default-features` pass per cargo root that carries a paired
module, five of the eight roots today.

What is measured:

* **It is a distinct feature unification**, so it is the shape this
  entry's own superseding note is about: it shares almost no artifacts
  with the `--all-features` passes and no cache configuration collapses
  the two. It is not a caching problem and will not be fixed as one.
* **Warm, gate plus self-test, base against head on one tree**: +33.4 s
  (4 vCPU, 2026-08-30, with three pass-3 self-test arms), +47.9 s (a
  second box, 2026-08-31, same arms), and **+58.2 s** (4 vCPU,
  2026-08-31, after two further arms and a merge of main: 86.9 s →
  145.1 s, of which +35.7 s is the gate and +22.5 s the mktemp
  fixtures). **The figure grew on every re-read**, which is the honest
  headline here: plan against the largest, and re-read rather than
  quoting the first.
* **Hosted, COLD**: `rustdoc (gate)` ran 331 s on run `33342678074`
  against 219/288/299 s on three contemporaneous PR runs whose cache
  also missed (restore ≤ 2 s in all four). Roughly +20–30%.
* **Hosted, WARM against WARM — the number this entry turns on, and it
  is measured rather than owed.** Two PR runs a cache-hit apart, same
  job shape (the non-gate steps are 69 s in both):

  | | run | cache restore | `rustdoc (gate)` | whole job | billed |
  |---|---|---|---|---|---|
  | merge base | `33342571322` | 14 s | 110 s | 179 s | **3** |
  | with pass 3 | `33346546955` | 13 s | 153 s | 222 s | **4** |

  **+43 s, and +1 billed minute.** The widening costs a minute on every
  code-tier PR run that reaches this job.

**And the thing that measurement turned up, which matters more than the
minute.** F6's headline — *"the gate is back inside 2 billed minutes"* —
had **already lapsed before this change**, for reasons that have nothing
to do with it: the merge-base job bills **3**, not 2, because the `fmt`
job has since grown a viewer-toolkit clippy pass (~41 s) and a wasm32
check (~19 s). F6 measured a 99 s job; the same job at the same warmth
is 179 s today. So the −1 this entry claims was spent by job growth
first and by pass 3 second, and the honest reading is that **a
billed-minute figure in this document is only true as of its own
measurement** — which is the argument under *Method* for keeping them by
hand, arriving as a worked example.

Still NOT measured: **the cache entry's size.** F6's +90 MB bought the
seven roots at ONE selection. Pass 3 adds a second fingerprint set to
five of them, in the same cached target directories. Nobody has read the
new figure.

If the minute is worth reclaiming, the lever is pass 3's root set — five
roots, of which `demos/tour` compiles the whole kernel — and not its
lint set, which is what makes the pass worth anything. The two job
growths above are the larger target and are nobody's row yet.

### 2026-09-03 — the rustdoc gate's other two passes demoted to the nightly

S-TCOST unit C2, Ev's approval in chat the same day, and it is F6's own
subject read one step further. F6 made the six excluded roots cheap by
caching them; the addendum above then recorded that the entry's −1 had
been spent twice over — once by growth elsewhere in the `fmt` job, once
by pass 3 — and named pass 3's root set as the lever if the minute were
ever worth reclaiming. This is that lever, taken at the SCHEDULE rather
than at the root set, so no coverage is dropped.

**What moved.** `scripts/doc-gate.sh` grew `--pr` / `--nightly` and a
`--scope`. ci.yml's `fmt` job runs `--pr`: the workspace pass alone, over
the change filter's `CARGO_SCOPE` (the closure on tier `closure`, the
whole workspace on tier `all`, the way `build` scopes). Pass 2 — one
`--no-deps` pass per cargo root the workspace excludes — and pass 3 —
`--no-default-features` over every root with a `not(feature)` half — are
nightly.yml's `rustdoc (gate, every root)`, ungated, on any night main
moved. `local-scripts/ci-local.sh` is unchanged and still runs all three
over every root.

**Argued against §*What is NOT sampled, and the rule*.** A broken
intra-doc link, a doc comment that stopped rendering, a `not(feature)`
half that no longer compiles: each PERSISTS in the tree until someone
fixes it, so a later run finds what a PR run would have. None of them is
a detector of ABSENCE. The parts of that gate which ARE about absence —
the two readers that refuse to report green over a tree they could not
read, and the derived root list whose whole subject is a root falling
silently out of coverage — live inside passes 2 and 3 and moved WITH
them, so they run in full every night rather than being left behind at a
cadence their guard does not share. That is the distinction the `k-lint`
entry above turns on, argued here rather than inherited.

**THE SCOPING IS A SECOND DEMOTION, and it gets its own row-by-row
sentence rather than riding the one above.** Pass 1 does not merely move;
it also narrows, from `--workspace` to the change filter's `CARGO_SCOPE`
on tier `closure`. So a workspace MEMBER outside the closure — one no
changed crate depends on — has its prose read on a PR run by nothing, and
is covered by the nightly alone. Against the rule that is the same
persistence case as the excluded roots, and it is weaker in one direction
and stronger in another. Weaker: a member's prose is likelier to be
edited by a PR than an excluded root's is — but a member whose OWN
sources changed is a seed and so is in its own closure by construction,
and what is skipped is a member nothing in the diff touches or depends
on. Stronger: what a doc link most often breaks on is a RENAMED or
DELETED item, and a rename in crate A that breaks a link in crate B puts
B in A's dependent closure, which is exactly what `CARGO_SCOPE` selects.
What genuinely escapes is a link broken by an edit to prose in an
unrelated member — ordinary persistence: it stays broken, and the nightly
reads every member. Tier `all` is unscoped, so an unclassifiable change
still documents everything.

**The cache moved with the passes.** `--print-roots --pr` prints `.`
alone, and the `fmt` job's `workspaces:` input is that: F6 taught the
cache about seven target directories because the job wrote seven, and
the job now writes one. The derivation is still ASKED FOR rather than
copied, so the cache's scope cannot drift from the passes that run.

**Billed minutes — COLD, and that is not the comparable number.** The
`fmt` job's rust-cache key hashes the job definition, so the first run
after any edit to that job is cold; F6 says so at its own entry and it
applies to this one. Run `33722478540` (this unit's opening run, head
`4ca9102a`): the job billed **6** (360 s), of which `rustdoc (gate)` was
246 s and the cache restore was a 10 s miss. Against the addendum's own
cold reading of 331 s for that step on run `33342678074`, the direction
is right and the magnitude is not yet the answer.

**Billed minutes — WARM AGAINST WARM, which is the reading this entry
turns on.** Run `33727294346` (head `416b94bf`, cache hit, restore 17 s)
against the addendum's two, whose method this copies:

| | run | `rustdoc (gate)` | non-gate steps | whole job | billed |
|---|---|---|---|---|---|
| merge base, before pass 3 | `33342571322` | 110 s | 69 s | 179 s | **3** |
| with pass 3, on every PR | `33346546955` | 153 s | 69 s | 222 s | **4** |
| this unit | `33727294346` | **110 s** | **69 s** | **179 s** | **3** |

**−1 billed minute per code-tier PR run**, and the shape of the number
is worth more than the minute: the job is back at the merge base's cost
to the second, with pass 3's coverage KEPT rather than dropped — it runs
nightly instead of per PR. The non-gate steps are 69 s in all three,
which is what makes the three rows comparable at all.

**And the nightly side is priced, because a demotion that books only its
saving is half a measurement.** `rustdoc (gate, every root)` runs all
three passes over all seven roots on a cache lane of its own, once a
night: **~3 billed**, derived from F6's warm all-seven job (99 s) plus
the addendum's pass-3 delta (+43 s), with one night at ~6-7 whenever that
lane's key rotates — the one-cold-run tax F6 records. So the ledger is
−1 per code-tier PR run against +~3 a night; at this repo's PR rate that
is a saving, and on a quiet day the `gate` job spends nothing at all. It
is DERIVED and not yet measured on a nightly; the row in the nightly
budget table below says so and asks for the re-read.

Two things the row does NOT claim. It is not F6's −1 recovered: that one
was spent by job growth as well as by pass 3, and this reading says
nothing about the growth. And 110 s is the gate plus the self-test on a
tree whose passes 2 and 3 no longer run here — that the total lands on
the pre-pass-3 figure exactly is a coincidence of two movements in
opposite directions (fewer passes, a longer self-test), not a
cancellation anyone designed.

**What the split does NOT buy back, said so it is not rediscovered.**
The `--selftest` is the half no cache reaches — every case plants a
fresh fixture under `mktemp -d` and cargo keys fingerprints on the
package path — and this unit made it LONGER, not shorter: it added an
arm per mode in both directions, a second fixture member, and three
refusal cases, because a mode nobody checks is a second gate nobody
checks. So the saving here is entirely in the real gate's passes, and
the self-test is now the larger share of that step. The lever on it is
still the one F6 named — running the cases in parallel — and it is still
nobody's row.


## What landed

* `db4f7ca` — `test-interval`'s 2x2 matrix (eps x shard) → one job,
  both eps rows, unsharded. Four legs billing 4 minutes to do ~1
  minute of work (12/15/17/20 s measured). The sharding's stated
  premise ("the unsharded interval default leg measured 6.1 min of
  execution, the run's slowest") predates the interval-only selection
  that cut the leg to a handful of tests. Independent per-eps verdicts
  are preserved by a failure-collecting loop; this also matches the
  local mirror, which never sharded these. **−3 billed min.**
* `95d0972` — `rustdoc (gate)` folded into the `rustfmt` job. Two jobs
  billing 2 minutes for ~50 s of work, most of it shared runner setup.
  Both keep their own step and step name; the doc gate runs even when
  rustfmt failed, so the verdicts stay independent. **−1 billed min.**
* `0768882` — the push-to-main run reduced to `filter` +
  `rebuild-latency` + `renders` (F3). **−~71 billed min per merged
  code PR**, and the largest single saving in this audit.
* `bb65cb9` — `persistence` and `band 4 corpus`: the 2-row eps matrix
  in each collapses into one job that compiles once and runs both eps
  rows against the same binary. These drive `cargo test`, so the
  matrix was paying for the same compile twice. **−4 billed min.**
* sccache rig, inert behind `vars.SCCACHE` (F4). **0 min until run.**
  *Read in 2026-09-03's trial window and kept off: it cannot cache a
  test binary, so the 0 stands for good. F4 carries the numbers.*
* `#921` — the `fmt` job's `Swatinem/rust-cache` told about all seven of
  the cargo roots the rustdoc gate documents, instead of the one it
  defaults to (F6). The list is not written into `ci.yml`: a step asks
  `scripts/doc-gate.sh --print-roots`, which runs the gate's own
  derivation, so the cache's scope cannot drift from the gate's coverage.
  **−1 billed min** — *spent, and then some: measured 2026-08-31 the job
  bills 3 at that state and 4 with the D180/D301 widening. Job growth
  unrelated to the gate took the first minute; pass 3 took the second.
  See F6's addendum.*

### 2026-08-22, the second pass — and it is a different KIND of change

The five items above tune what a job costs. The six below change **what
runs at all**, or **when**, and each carries the argument for why that is
sound rather than a cut. They land together on one branch.

* **`persistence` and `band 4 corpus` DELETED** (not merged again —
  deleted). `bb65cb9` above had just collapsed each one's ε matrix; the
  ε *sampling* two days later made the jobs themselves redundant, and
  the ordering is the whole story. Every module they named —
  `m4_pr6_roundtrip::`, `m4_pr6_floats::`, `m4_pr6_golden::`,
  `m4_pr6_eps_diff::`, the D6.3 schema/corruption rows, `m4_pr8_corpus::`
  — is an ordinary `#[test]` in the nextest archive that the `test` job
  already runs at the ε THIS RUN DREW. The two jobs re-ran them at 1e-6
  AND 1e-12 unconditionally, which is not extra coverage: it is the ε
  sampling defeated for exactly those modules. **No replacement
  mechanism** (Ev: *"no need to make any special attempt to keep
  m4_pr8_corpus visible. just make it a normal test"*) — no filter
  expression, no named row, no doc pointer. `m4_pr6_eps_diff::` never
  needed the loop at all: it re-execs itself per ε, which is the only way
  two ε values can exist in one audit. **−4 billed min.**
* **`k-lint (gate)` samples ONE of its five feature unifications per
  run**, drawn under a salt of its own. See the superseded-cache block in
  F6 above for why this and not a cache, and for the per-row
  absence-detector audit that had to come first. **−7 to −8 billed min**,
  the largest single item in this pass.
* **`watertight` → the nightly** (Ev, explicit). A persistence-detector
  with one solo red in 37 days, and that one was a rustup outage.
  **−1 to −2 billed min.**
* **`rebuild latency (reporting)` SPLIT.** The wall-clock table moved to
  the nightly; its ε-independent structural pins (counted reuse, the
  corpus manifest's nodes/cone) became an ordinary non-`#[ignore]`d
  `#[test]` in `editor-core`, inside the archive. That half now gates
  EVERY PR at about zero marginal cost, where the job ran only on
  `run_editor_core` — so this ADDS coverage while removing the minutes.
  **−2 billed min on the PR side, and −2 per merge**: the push-to-main
  run is now `filter` + `renders`.
* **`render lanes`: five jobs → two.** `uv` and `wild` fold into the tour
  job under one `CARGO_TARGET_DIR` (measured first: the two roots build
  58 and 57 packages, differing only in each root's own leaf crate, no
  version mismatch anywhere — which is why this is NOT the
  `CARGO_TARGET_DIR` variant F6 rejected, where the fingerprints
  genuinely differed); the two montage legs merge behind one 821 MB
  FreeCAD cache restore, one apt install and one setup, with a
  failure-collecting loop preserving both verdicts. Every lane still
  produces every artifact under the same name. **−2.8 to −4.2 billed
  min.**
* **A nightly lane exists** (`.github/workflows/nightly.yml`), holding
  `watertight`, the rebuild-latency table, the demoted tests (seven at
  the time of writing, DERIVED rather than listed — see *Demoting a test
  to the nightly* below; measured on the real tree, the same derivation
  correctly leaves out all 16 pre-existing plain `#[ignore]`s) and the
  opt-level calibration below. It does not run when main has not moved
  since it last ran — an append-only `nightly/<epoch>-<sha>` tag, with
  the tier question handed to `scripts/ci-filter.py` rather than to a
  second classifier. **This is not the scheduled full run on main that
  F3 left owed and Ev declined**; see *What did not land* below, where
  that entry now says why the two are different questions.

**Two things in that list are coverage ADDITIONS, and it is worth saying
so in a document about spending less:** the rebuild-latency structural
pins now gate every PR rather than only `run_editor_core` runs, and the
seven newly demoted tests — which the gate skips by construction — run in
the nightly, where before this branch they would have run nowhere at all.

## The wall-clock cost of the four job merges: zero

Measured, not assumed. The merges add serial work to four jobs, and
none of them is near the critical path (`build + archive (interval)` at
12.4 min, then the longest test leg, ending at 13.75):

| merged job | was (parallel max) | now (serial) | on critical path? |
|---|---|---|---|
| test (interval) | 20 s | ~40 s | no — ends ~12.9 of 13.75 |
| rustfmt + rustdoc | 46 s | ~54 s | no — ends ~1.5 |
| persistence | 80 s | ~85 s | no |
| band 4 corpus | 78 s | ~82 s | no |

The persistence and corpus figures carry an inferred split: their
`cargo test` step is 42–47 s of compile *plus* run, and the follow-on
steps rounded to 0 s, so the added second-ε execution cannot be
separated from the log. The bound is what matters — the extra ε row
costs strictly less than the whole 46 s step, so even at the absurd
worst case those jobs reach ~126 s, still six times under the critical
path. **The run's wall clock does not move.**

(The `rustfmt + rustdoc` row moved again with **#840**, which sites the
#807 wasm32 guard in that same job: `fmt` is now `rustfmt + rustdoc
(gate) + wasm32`. **Two readings, and they disagree on the billed
cost**: cold, 64 s without the guard and 84 s with — both 2 billed
minutes, so +0; warm, 53 s without and 66 s with — 1 billed minute
against 2, so **+1**. This job's baseline STRADDLES the 60 s boundary
(`rustdoc (gate)` alone swings 22-34 s), so the guard bills 0 or 1
depending on which side the run lands. A job of its own would bill 1
always. It stays off the critical path either way: 13-20 s of guard
against a 12.4 min `build + archive (interval)`.

A caution this table earns: **a merged job sitting near a minute
boundary has an unstable billed cost**, and a single measurement of one
will read as a flat number when it is not. Both of the merges above
that land "inside one billed minute" are worth re-reading with that in
mind.)

**2026-08-22, and it settles that instability the expensive way.** The
rustdoc gate stopped being `cargo doc --workspace` and now also
documents the six cargo roots the workspace excludes — `demos/tour`,
`demos/wild`, the three `tools/` crates and `interval-transcendentals`
— plus a `--selftest` in front of it (D40/D41). Measured on two
code-tier runs, warm cache both: `rustdoc (gate)` **34 s → 87 s**, and
the `fmt` job **87 s → 135 s**, which is **2 → 3 billed minutes**. It
does not straddle any more; it bills 3. The two `cargo doc` steps this
deleted from the `k-lint` job gave back nothing measurable (that job
builds those crates for its own clippy and test rows, so their doc pass
was reusing warm artifacts at ~1 s), so the change is **+1 billed minute
per code-tier PR run**, not a transfer. The reason it costs what it
costs is that the `fmt` job builds all six of those roots from cold —
`Swatinem/rust-cache` in that job caches the kernel workspace's
`target/` and not the excluded roots'. Declaring those roots to that
action (its `workspaces:` input), or pointing the whole gate at one
`CARGO_TARGET_DIR`, is the lever if this minute is ever worth
collecting; neither was measured here — **F6 measures both**, and
collects the minute. Still off the critical path:
2.3 min against a 12 min `build + archive (interval)`.

**These two figures are re-taken by nothing, and that is a decision.**
No register in `ci.yml` re-measures them — R4 is the Python suite and
has nothing to do with this job — so if the gate's cost drifts, this
paragraph goes quietly stale the way a hand-written count does. The
reason it is left that way rather than guarded: a *billed-minute* figure
is not a property of the tree, it is a property of a runner on a day,
and the paragraph directly above says why — this job straddles a minute
boundary, so a guard pinning 3 would red on warm-cache noise and a guard
pinning "≤ 3" would pass through the entire drift it exists to catch.
There is nothing to assert that is both true and useful, which is the
honest shape of Q6 here rather than an oversight. What IS derived is the
root count: the gate's success line reports the number of roots the run
actually documented, so the six above is checkable against any run's log
and does not depend on this sentence being maintained. The paragraph
below re-takes the timings BY HAND, which is what maintaining an
unguardable figure looks like — a deliberate act carrying the run id it
came from, not a number that keeps itself true.

**Re-measured after two widenings, on run `32553404730`** (warm cache,
code tier, same method): `--examples` was added to every pass, and pass 2 went
from default features to `--all-features` on every root but
`interval-transcendentals` — so the `fmt` job now also builds `demos/tour` with
`probe` and `budget` on. `rustdoc (gate)` **87 s → 142 s** and the `fmt` job
**135 s → 193 s**, which is **3 → 4 billed minutes**: another **+1 billed
minute per code-tier PR run**, on top of the +1 above. The step breakdown from
that run — rustfmt 3 s, rustdoc gate 142 s, wasm32 19 s, setup and cache 25 s —
says where it all is, and it is all in the gate.

Both widenings were taken because each closed a hole the gate was silently
green over, and each turned up a live break the moment it opened: an unresolved
link in `crates/step-export`'s example tree, which no rustdoc lint had ever
read, and another in `demos/tour/src/tessbudget.rs`, which exists only under
the `budget` feature the old default-features pass never turned on. Still off
the critical path: 3.2 min against a 12 min `build + archive (interval)`. The
`workspaces:`/`CARGO_TARGET_DIR` lever named above is now worth proportionally
more. **Both halves of it are measured in F6**, which lands the `workspaces:`
one (`fmt` back to 2 billed minutes) and rejects the `CARGO_TARGET_DIR` one.

## Where that leaves a code-tier PR

Roughly **87 → 79 billed minutes** on the pull_request side (the four
job merges), plus the post-merge run dropping from ~87 to ~16. For a
PR that lands after N pushes, total spend goes from `87N + 87` to
`79N + 16`.

The PR-side figure is deliberately modest. The three biggest line
items — the two build jobs at 12 each and `k-lint` at 10 — are the
critical path and were not touched; `renders` at 12 is F1, and is a
feature rather than waste. What remains, in rough order of size, is
F5 (policy), F1's paired change to `render-hosted.sh`, and whatever
F4 measures.

**Then configuration sampling took it to ~62**, which is the current
baseline: the derivation is in *Expected cost, derived not measured*
below (~79 → ~60 on that section's arithmetic; ~62 is the same figure
carrying the two later `fmt`-job widenings). The draft skip cuts the
COUNT of full runs on top of that and multiplies against everything here.

### And the 2026-08-22 second pass takes it to ~40

Derived the same way — per code-tier PR run, against the ~62 above.
Every line is the item from *What landed* directly above; none of these
is measured on a runner yet, and the first few real runs of the branch
are what settles them.

| item | Δ billed |
|---|---|
| `persistence` + `band 4 corpus` deleted | −4 |
| `k-lint` samples 1 of 5 unifications | −7 to −8 |
| `watertight` → nightly | −1 to −2 |
| `rebuild latency` split (table → nightly) | −2 |
| render lanes: five jobs → two | −2.8 to −4.2 |
| **total** | **−17 to −20**, i.e. **~62 → ~40** |

The push-to-main run drops again with the rebuild-latency move, from
~16 to ~14, because that job was one of the three it still carried.

**What this now costs elsewhere, and it is not free.** Derived, not
measured, and the largest line is not the one you would guess:

| nightly job | billed | note |
|---|---|---|
| `demoted` | ~0 today | TWO workspace builds — the gate-side listing and the `--cfg nightly_suite` one — then ~64 s of tests, but **only once a test is demoted**: the markers were reverted (`bbdaebf`), so the job short-circuits. ~11 when the population is non-empty |
| `watertight` | ~2 | |
| `rebuild latency` | ~2 | its own compile, deliberately not the archive |
| `gate` + `record` | ~2 | |
| `opt-level` | ~2 | the free arm only; **+~25-30 one night a week** when the two measured arms run |
| `rustdoc (gate, every root)` | ~3 | S-TCOST C2. DERIVED, not yet measured on a nightly: F6's warm all-seven reading is a 99 s job and the addendum's pass-3 delta is +43 s, so ~142 s. Its own cache lane (`nightly-rustdoc-roots`) is warm night to night; a key rotation costs one night at ~6-7, the same one-cold-run tax F6 records. Re-read it from the first nightly run. |
| `corrupt input (release profile)` | ~2 | S-TCOST C1. The job's own audit-table line, unchanged by the move: it is one `-p topo --lib` release compile and five rows that execute in milliseconds. Read at 98 s / 2 billed on run `33722922975`, where it was still a ci.yml job on a comparable tree. |
| `python suite (ungated re-take)` | ~2 | S-TCOST C3. The job's own line, re-read at 120 s / 2 billed on run `33722922975` where it was still ci.yml's on a comparable tree; 67 s of that is the wheel, on a cache lane (`nightly-python`) of its own. |
| **an ordinary night** | **~15** | **~41 on a calibration night** (both figures assume `demoted` is short-circuited; add ~11 once anything is demoted). Was ~8 before the three S-TCOST C-units above joined this lane on 2026-09-03; each of them books a saving on the PR side against its row here, and `rustdoc (gate, every root)`'s ~3 is the one figure still DERIVED rather than read off a nightly |

**`demoted` is the largest single line here, and the reason is
structural rather than sloppy**: the selection is a difference between
two listings, and a listing is a build. (It read *over half of it* until
2026-09-03, and that was true of a ~8-minute night against `demoted`'s
~11: 11 of 19. The three C-units above take the ordinary night to ~15,
so it is now ~11 of 26 — still the biggest row by some way, and no
longer a majority. The sentence is corrected rather than left standing,
because a figure in this document that disagrees with its own table is
the drift the document is about.) The gate-side one is therefore taken at **opt-0** —
nothing is executed from it, and the selection reads test NAMES and
`ignored` FLAGS, neither of which an optimisation level can move — which
is ~130 s against the ~430 s the opt-2 build cost (this document's own
F-numbers).

**The run itself tracks the gate** — opt-2 until 2026-08-25, opt-1 since.
A demoted test is one the gate would otherwise run, so what this job has
to answer is whether it still passes, and what it costs, *in the
configuration we actually use*. Pinning it to a level the gate has
stopped running would answer a question nobody has (Ev, 2026-08-25): a
cost measured in a configuration we do not use is not a cost anyone can
act on. That also shrinks the gate-side listing's saving rather than
removing it — against opt-1 the gap is ~164 s on a 4-core sweep (143 s →
307 s) instead of ~300 s against opt-2, and the hosted opt-1 build figure
is what the `opt-level` lane's free arm now produces on every nightly.

**And this lane currently selects nothing.** The seven demotion markers
were reverted in `bbdaebf` ("the nightly job is net negative until the
opt-0 flip is real"), so the job short-circuits on its
`are any tests demoted at all?` step and the ~11 billed minutes in the
table above are what it *would* cost once a test is demoted again, not
what it bills today.

It runs **only on days main actually moved**, and nothing on it gates a
merge. Against a repo doing ~13 code-tier runs an hour during active
work, a lane billing about a third of one PR run per day is the trade the
demotion argument rests on — stated with a number rather than assumed.

## What did not land, and why

* **render lanes off PR runs** — F1. Breaks `render-hosted.sh`'s
  default path. Needs the paired change to that script.
* **merging the default-build test shards** — F2. Costs ~7% run
  latency to save 1 minute.
* **merging `clippy` with `clippy (interval)`** — the two compile
  different feature unifications and share no artifacts, so the merge
  buys only one runner setup (~1 billed min) while serialising ~4
  minutes into a single job. Not worth it.
* **a scheduled full run on main** — DECLINED (Ev, 2026-08-22), not
  owed. The next PR's merge-ref is main plus that branch and tests the
  landed tree anyway. See F3 for the residue that is accepted with it.

  *Still declined after the 2026-08-22 nightly lane landed, and the two
  are not the same proposal.* A scheduled FULL run re-gates a tree the
  next PR will gate anyway, at the price of a whole gate per period
  whether or not anything landed. The nightly runs what NO PR run can:
  rows deliberately taken off the per-PR gate (`watertight`, the demoted
  tests), a measurement lane that wants a cadence rather than a merge
  (rebuild latency), and a calibration whose subject is the runner itself.
  It also does not fire on a period — it fires on main having moved, which
  is the property the declined proposal lacked.
* **draft-PR skip** — F5. Policy decision.
* **`RUN_EDITOR_CORE` retired** — considered when `persistence`, `band 4
  corpus` and `rebuild latency` all left ci.yml, and NOT done: the
  `test-interval` job's two named interval rows still read it, as does
  the local half. A signal with live consumers is not dead plumbing.
* **running only the demoted tests via a central filterset** in
  `.config/nextest.toml` — rejected on the same ground the marker is
  sited at the test: a second list drifts out of sync with the tests it
  names. The set is derived instead, as the difference between two
  `cargo nextest list` outputs (`scripts/nightly-only-selection.py`,
  modelled on `scripts/interval-only-selection.py`) — no list, and a test
  whose marker is deleted leaves the set on the next run.

## 2026-08-22 — configuration sampling, and the draft skip (F5)

### SUPERSEDED IN FULL, 2026-09-04 — full runs reinstated on every dimension

**Everything in this section below this block describes the regime that ran
from 2026-08-22 to 2026-09-04.** The lane and ε draws are gone: a code-tier
run gates every point of {default, `interval`} x {default, 1e-6, 1e-12}, as
two archives and twelve `test (…)` jobs. **The k-lint unification draw is
gone too, later the same day**: all five unifications run as five
`k-lint (gate, <row>)` matrix legs. The first version of this block said the
k-lint draw was NOT affected, and it was not — for about eighteen hours. Its
measurement is in the 2026-09-04 section at the foot of this file.

**Ev's authorisation, in chat, 2026-09-04**: *"feel free to reinstate full
runs instead of sampling"*, on the reasoning that *"CI is weakened right now
because of sampling only certain configurations to run … undoing that
sampling now that actions minutes are much cheaper is probably a good idea"*.
The premise this whole document opens with — an Actions allowance being
consumed faster than the work justified — died when the repository went
public on 2026-09-03.

**The measurement.** Population: every `CI` workflow run created
`2026-09-04T04:00:00Z`–`2026-09-04T07:52:14Z` with `conclusion` ∈ {success,
failure} (cancelled excluded — a cancelled run's job set is a truncation, not
a sample): **156 runs, 72 code-tier** (a code-tier run is one carrying a
non-skipped `build + archive` job). Durations are per-job wall time from the
jobs API; nothing here is a BILLED minute, because a public repository's
standard-runner minutes are not billed.

**That population was re-derived after two readers disagreed with it**, and
both earlier counts were wrong. This document first said 155/71 and a review
said 154/72. The window was re-fetched once every run in it had CONCLUDED:
**156/72**, with `build + archive (interval)` appearing on **39** runs and
`(default)` on 38. The 155 came from a snapshot taken while one run in the
window was still in flight, so it had no conclusion yet and was dropped. **A
population counted from a window that is still open is a different population
five minutes later**, which is the only interesting thing about the
disagreement.

| | today (sampled) | un-sampled |
|---|---|---|
| job-minutes, median code-tier run | **24.7** (n=72) | **~40** |
| critical path, median code-tier run | **399 s** default draw / **456 s** interval draw (pooled tiers) | see CORRECTION 2 — this row understates it |
| `build + archive (default)` | 273 s scoped (n=31) / 322 s tier=all (n=7) | unchanged |
| `build + archive (interval)` | 366 s scoped (n=34) / 400 s tier=all (n=5) | unchanged |
| one `test (…)` leg | 42 s / 55 s scoped (n=54 / 60); 46 s / 94 s tier=all (n=14 / 8) | unchanged |
| `clippy` / `clippy + doc-tests (interval)` | 72 s / 100 s | unchanged |

**It is not a 6x multiplier, and the reason is the archive.** ε is runtime
env, read by bit-identical binaries, so the six points are TWO builds and
TWELVE test legs. Summing the configuration-dependent jobs at the medians
above: un-sampled = 646 s of builds + 172 s of lint + 618 s of test legs =
**1436 s**; today's expectation over the 50/50 draw = **512 s**. So
**+924 s = +15.4 job-minutes per code-tier run**, on a median run of 24.5 —
so **24.5 → ~40**. **That pooled figure is superseded by the per-tier split in
CORRECTION 1 below**, which the gate run forced; it is left standing so the
correction has something to correct. Its inputs are also the first,
still-open-window population (155/71) rather than the closed-window one
(156/72) the measurement paragraph now states.

**Why the component sum and not a run-total comparison.** The observed
medians split by drawn lane are 21.1 (default, n=33) and 25.4 (interval,
n=33), a gap of 4.3 job-minutes where the per-job medians account for only
2.3. The rest is CONFOUNDED, not measurement error: a diff under
`interval-transcendentals/` PINNED the interval lane, so the interval-drawn
set is enriched in runs that are `TIER=all` and carry the oracle and backend
rows for reasons that have nothing to do with the lane. Summing the jobs the
lane actually decides avoids that; comparing run totals does not.

**CORRECTION, 2026-09-04, from this unit's own gate run.** The figure first
published here was **+15.4 job-minutes per code-tier run** (run-weighted, it
is +15.6), derived from per-job medians pooled across both tiers. **The gate run falsified it for the
tier the gate run is in, and the measurement wins.** Run `33853141826`
(`dc415acf`, green, 32 jobs, twelve `test (…)`) is a TIER=all un-sampled run
at **54.0 job-minutes and 619 s wall**, against a TIER=all sampled median of
**30.1 job-minutes and 437 s** (n=10 in the same window): **+23.9 job-minutes
and +182 s**.

Re-derived per tier, which is what the pooled figure was hiding:

| | n (sampled) | un-sampled config jobs | sampled expectation | delta |
|---|---|---|---|---|
| **TIER=closure** (61 of 72 code-tier runs) | 61 | 1394 s | 502 s | **+14.9 job-min** |
| **TIER=all** (11 of 72) | 11 | 1756 s | 592 s | **+19.4 job-min** |

Run-weighted: (61·14.9 + 11·19.4)/72 = **+15.6 job-min**, which is what the
pooled figure was measuring without saying so.

The tiers differ because TIER=all builds and runs unscoped: its interval test
legs median 89 s against 55 s scoped, and its archives 322/386 s against
273/366 s.

**The gate run is 5.0 job-minutes above even the TIER=all derivation, and I am
not explaining that away.** Its own `build + archive (interval)` took 425 s
against that group's 386 s median and its `rustfmt + rustdoc` 370 s against
~133 s on a typical run, so **n=1 against a median is not like-for-like** —
but the honest reading is that the derivation is a floor and the first
un-sampled run landed above it. Whether twelve concurrent legs slow each other
is **not measured**; one run cannot tell that from ordinary variance.

**Second data point, and it moves the reading toward the derivation.** Run
`33854219517` (`b2159cf5`, green, 32 jobs, twelve `test (…)`) is the same
TIER=all un-sampled shape at **44.4 job-minutes and 596 s wall**. So the two
un-sampled runs are 54.0 and 44.4 against a 30.1 sampled median: **+23.9 and
+14.3, mean +19.1**, and the +19.4 derivation sits between them. n=2 is still
n=2; no further runs are being chased, and the derivation is what to quote.

**What survives unchanged**: TIER=closure is 86 % of code-tier runs and its
**+14.9** still agrees with PR 1796's independently derived **+15.6** (that
figure is tier-blind too, in the same way this one's first version was). The
shape of the argument — two builds and twelve test legs, not six builds — is
what the gate run confirms.

**Two currencies, and the second one depends on the window.** At this
window's **18.6 code-tier runs/hour** that is **+290 job-minutes/hour**
against a measured 494 job-min/h of all CI. The rate is not a constant: PR
1796's population (2026-09-03T15:20Z–2026-09-04T05:47Z, 14.45 h) has 10.3
code-tier runs/hour and prices the same change at +161 job-min/h. **The two
figures agree on the per-run number — +15.6 run-weighted here against +15.6
there — and differ only on how busy an hour is.** That is agreement, not
independent confirmation: same derivation shape, same jobs API, same
infrastructure, two windows, with component medians differing by up to ~30 s
in both directions in a way that happens to cancel. Quote the per-run number;
derive the hourly one from the window you care about.

**Critical path: superseded by CORRECTION 2 below, which the un-sampled runs
forced.** What stood here: every run now takes the interval lane's path
because that archive is the slower of the two, a run that would have drawn
`interval` is unchanged, and *"the added ε legs cost ~0 wall: they start
together behind an archive that was already being built"* — expected **+28 s,
about +7%**. The first clause holds. The ε-leg clause does not.

**A cross-check, and why it is not the headline.** Five runs in the window
already carried `CI-Config: lane=both` — the un-sampled lane at one ε — and
their median is 34.8 job-minutes and 516 s wall (run `33848247472` is one).
Median-to-median that is +11.5 job-min for the lane alone, against the +8.5
the per-job medians derive; n=5, not a random sample (a lane asks for
`lane=both` when it is doing something unusual), so the component derivation
above is the one to quote.

**What this does and does not buy.** It closes the gap where a defect at an
undrawn point merges green and surfaces on a stranger's branch — each point
gated ~1 run in 6, or 9-31% of code-tier runs by PR 1796's count. It does
NOT address a composition that no run ever compiles (PR 1796's subject: two
independently-green PRs whose merge is red), and it does not add a check that
runs nowhere. **No claim is made that it would have caught a specific past
red**: of the five recorded main-reds PR 1796 enumerated, zero are
attributable to the lane/ε draw — and that population is biased against
exactly this class, because a defect at an undrawn point leaves no record
until something draws it. The argument is price and exposure, not history.

### CORRECTION 2, 2026-09-04 — the wall-clock claim was wrong, and the ε legs are why

**The sentence "the added ε legs cost ~0 wall" was false and is withdrawn.**
The legs do start together — verified on run `33848247472` — but **wall follows
the max over them, not the overlap**, and six legs have a larger maximum than
two. The critical path's last job is named and it is the same one in all three
un-sampled runs: **`test (interval, eps = default, 1/2)`** — the first ε row's
shard 1, which also carries the two `editor-core` steps — at **156 / 151 /
135 s**, ending at **t+619 / t+596 / t+519 s**, which is each run's wall exactly.

Measured, against TIER=all runs of the same window split by the lane they drew:

| | sampled | un-sampled (n=3) | delta |
|---|---|---|---|
| would have drawn `interval` | **576 s** (n=4) | **596 s** (median) | **+20 s** — this is the ε legs' own cost, the archive being common to both |
| would have drawn `default` | **424 s** (n=6) | **596 s** | **+172 s** — mostly the interval archive |
| expectation over the 50/50 | ~500 s | **596 s** | **≈ +96 s** |

So the honest figure for a TIER=all run is **about +96 s of critical path in
expectation, of which ~20 s is the ε legs and the rest is the interval
archive** — not the "+28 s, and ~0 from the ε legs" first published here. The
n on every cell is small (3 un-sampled runs against 4 and 6 sampled ones) and
the ε-leg term is the smallest and least certain of them; what is not
uncertain is that it is **not zero**, and that the trade was described as
free on the strength of the overlap when the overlap was never the question.

**One consequence beyond coverage**: a GitHub merge queue's required checks
are named, so un-sampling is a precondition for the queue PR 1796 recommends
trialling.

**`local-scripts/ci-local.sh` is no longer "the only lane that runs every
point on one tree"** — the sentence below saying so was true for thirteen
days. Local still runs all five k-lint unifications and its opt-in
`--nightly` row; hosted now matches it on the lane and ε.
Ev's proposal, and the reason it is a separate section rather than a
finding: the audit above tuned what each job costs, and this changes
**what a run gates**. A code-tier run used to execute every point of
{default features, `interval`} x {default eps, 1e-6, 1e-12}. It now
gates ONE, drawn from the head SHA. The premise is that those six points
almost always agree — which the `interval` additivity gate
(`check-interval-cfg-additive.py`) and the runtime-eps contract already
assert — so repetition covers the matrix: at the ~60 runs/hour measured
under **Volume**, a break confined to one point surfaces within minutes.
Nothing is shipped, so a briefly red main is affordable.

### Why the draw is seeded, not random

`scripts/ci-filter.py --seed <head sha>`, choice = `sha256(salt || seed)
% n`, salted per dimension so lane and eps draw independently (an
unsalted second draw ties eps to lane and leaves 2 of the 6 points
unreachable). Two properties, both load-bearing:

* **A re-run of the same commit draws the same point.** Under true
  randomness a re-run of a red gate can come back green on a different
  point. That reads as a flake and teaches a re-run habit that launders
  real failures — the one failure mode that would make this change cost
  more than it saves.
* **The point is recoverable from the SHA alone**, so "which
  configuration gated this commit" is answerable during a bisect without
  the run's logs.

### How often the points actually disagree — one verified instance

The premise above says the six points "almost always" agree. That is not
the same as "always", and the counterexample is same-day: **run
`32556372010`** (PR #910's fix pass). Of its 32 jobs exactly one failed —
`test (eps = 1e-6, 2/2)` — while `default` and `1e-12` both decided the
same fixture cleanly. The cause was an adopted test's fixture margin
(`chord_side`, 1.0000000000282557e-6) sitting inside 1e-6's zero band,
and diagnosis found the fixture could not clear the whole matrix at any
parameter value.

**Read the premise as: disagreements are ε-band fixtures, they exist in
practice, and this codebase produces them** — its tests deliberately probe
bands, so a margin engineered near one ε's band is a recurring class
rather than a freak.

What sampling costs on that class is bounded and already priced in. Such a
break is caught pre-merge on the draws that hit the offending ε — **1 in 3
for this instance, not 1 in 6**, because the interval lane now runs the
whole suite, so an ungated test like this one runs on either lane and only
the ε draw matters. The other two draws merge it and it surfaces on main,
which is exactly what "nothing is shipped, so a briefly red main is
affordable" buys. The persistence argument applies unchanged: the fixture
stays broken, so a later draw finds it.

**Two consequences for anything that reads a green check.** "PR checks
green" now attests one point. A review verdict issued "conditional on
green", or a merge-row battery cell, should say WHICH point gated — the
job names carry it (`test (eps = 1e-6, 1/2)`) and the SHA-recoverable draw
makes it derivable after the fact. **Since 2026-08-28 that last clause has
an exception**: a run whose point was ASKED FOR is not derivable from the
SHA, which is why the request is recorded in the run itself
(`CONFIG_SOURCE`, printed by a step of its own). See *asking for a point
instead of drawing one*, below.

### What is NOT sampled, and the rule

Sampling is sound for a detector whose subject **persists in the tree**:
a test red at 1e-12 stays red, so a later draw still finds it. It is
unsound for a detector of **absence** — a check dropped from one half of
CI, a gate sited where it cannot fire — because an absence leaves no
future red for a later draw to catch, and the thing it detects merges
silently, once.

So `mirror` is not sampled (and structurally cannot be:
`check-ci-mirror-parity.py` requires it to run on every tier, since its
own inputs are the docs-tier paths on which every `if: run_build` job
skips). Neither are `discipline`, `fmt`, the python suite or the render
lanes.

**`k-lint` WAS in that list and no longer is, as of later the same day.**
Its five feature unifications are sampled one per run, under a third
salt. Nothing about the rule above changed — the rule is what licensed
it, and each of the five was audited against it individually rather than
as a group, because "this job is mostly persistence-detectors" is not an
argument about the row that is not. The audit and its result are in the
block quote inside F6; the short form is that the row which looked like
an absence-detector (the probe census) is not one, because the half of
that census which would notice a probe suite *disappearing* runs in
`discipline`, which is unsampled. **A future entry to this list has to be
argued the same way**: per row, against absence, with the answer written
down — not by inheriting `k-lint`'s.

### The interval lane runs the whole suite again

This reverses the 2026-08-13 interval-only selection **on the hosted
half only**. That selection subtracted the tests the default legs had
already run in the same run; a sampled run draws one lane, so on an
interval draw those legs do not exist and their ~93% of the suite would
be gated by nothing. `scripts/interval-only-selection.py` keeps exactly
one caller — `local-scripts/ci-local.sh`, which still runs both lanes on
one tree — and is declared in that script's `MIRROR_EXEMPT` with the
reason.

**`local-scripts/ci-local.sh` WAS then the only lane that runs every point
on one tree**, and is deliberately not sampled: nothing bills it by the
minute. (Superseded 2026-09-04 — hosted runs every lane and ε point too;
local still adds all five k-lint rows and its `--nightly` row.)

### Expected cost, derived not measured

Per code-tier PR run, against the ~79 the audit above leaves:

| | today | default draw | interval draw |
|---|---|---|---|
| build / build-interval | 24 | 12 | 12 |
| test legs | 8 + 1 | 4 | 6 |
| clippy / lint-interval | 5 | 2 | 3 |
| **compile-mode subtotal** | **38** | **18** | **21** |

So **~79 -> ~60 billed minutes**, about -23%, and the draw that skips the
interval build also shortens the critical path. The draft skip (F5, now
landed) then cuts the *count* of full runs, which multiplies against
everything above and is the larger lever of the two.

### Two things that were re-asked and did NOT change

* **Sharding stays**, in both lanes. F2's arithmetic is unchanged by the
  eps cut: merging the two shards saves 1 billed minute and costs ~66 s
  of latency on the default lane, ~117 s on the interval lane — and the
  run leg now sits directly behind the only build job on the critical
  path, with nothing to hide behind. One minute of ~60 against ~9-14% of
  wall clock is the wrong side of the trade.
* **opt-level 2 stays** — *superseded 2026-08-25: the tree is at opt-level 1.
  See the block at the foot of item 4 in the ranked list below, and `ci.yml`'s
  OPT LEVEL note for the decision itself. The bullet is kept as written
  because the reasoning it records is what the flip had to answer.* Re-asked
  because the verdict rested on
  amortising one compile over ten run legs and a sampled run has one.
  Writing E for the row's opt-2 execution and r for the opt-0/opt-2
  ratio, opt-2 wins while `E > (archive_2 - archive_0) / (r - 1)`:
  break-even is 56 s against a ~119 s row on the default lane (r =
  6.46), and 78 s against a ~234 s row on the interval lane (r = 7.08).
  Margins of ~2x and ~3x. The interval lane is why it is not close, and
  it got *more* expensive to execute under sampling, not less.

  > **THE `r` IN THAT PARAGRAPH IS NOW IN DOUBT — 2026-08-22, later the
  > same day.** A census re-measured the same ratio at **4.95 (default)
  > and 4.99 (interval)** on a 4-core AVX-512 guest, against the 6.46 and
  > 7.08 above. That removes about 30% of the quantity the verdict rests
  > on and turns "margins of ~2x and ~3x" into **0.94x and 0.91x** — opt-0
  > winning outright, with no cuts at all.
  >
  > **This does NOT flip anything, and the reason is the whole point.**
  > The census box is not the box CI runs on: a 4-core AVX-512 guest
  > against a 2-vCPU hosted runner, and `r` is exactly the kind of number
  > that does not transfer between machines — the figures above already
  > say so in the other direction (the compile penalty measured 4.58x
  > hosted against 3.24x locally, and the execution win 7.1x hosted
  > against 2.29x locally). Two boxes disagreeing about `r` is expected;
  > what the census establishes is that **the number this verdict turns
  > on has never been measured on the runner it is about.**
  >
  > So the reading of this bullet is: opt-level 2 stays, *and its
  > justification is now known to be unverified rather than merely
  > untested.* `nightly.yml`'s `opt-level calibration` job settles it on
  > the real box, and records `r`, `E2`, `a2 − a0` and the build/total
  > split beside each verdict so the next reader can tell when the
  > conclusion expired rather than inheriting a bare `opt-level = 2`.
  > **It has expired once already**: opt-2 (#449) was itself a reversal of
  > an earlier opt-0 verdict (#52/#53) whose premises went stale, which is
  > the strongest available argument for measuring it on a cadence instead
  > of writing it down once.

### Demoting a test to the nightly, and why it needed no roster

The nightly lane exists (above); this is the mechanism that puts a test
in it, recorded here because it is a decision about **what a PR run
gates**, which is this section's subject rather than the audit's.

At the test, with its reason:

    #[cfg_attr(not(nightly_suite), ignore = "nightly-only: <reason>")]

and the nightly builds with `RUSTFLAGS="--cfg nightly_suite"`. At the
gate the attribute is present and the test is skipped; in the nightly it
vanishes and the test is ordinary.

**Ev's constraint, and it holds by construction rather than by a
list**: tests that are ALREADY plain `#[ignore]` — reporting rows,
instruments, tests only valid as the sole test in a process — must stay
unexecuted in the nightly too. So the nightly must never pass
`--run-ignored` in any spelling, and it does not need to: under the cfg
the demoted tests are ordinary tests that a plain filtered run executes,
while a plain `#[ignore]` still carries its attribute and is not in the
selection.

**Which tests, without a roster.** The marker lives at the test — the
same argument `check-ci-mirror-parity.py` makes for siting its
`NO LOCAL MIRROR` reasons at the job — so there is nothing to read the
set off, and a central filterset in `.config/nextest.toml` would be a
second list that drifts. It is DERIVED instead, exactly as
`scripts/interval-only-selection.py` derives the interval feature's own
tests: list the suite twice, subtract. The one detail that is easy to get
wrong, and that a name-set difference gets wrong silently, is that
`cargo nextest list` reports ignored tests too — so the difference is
over the `ignored` FLAG (`{t : ignored at the gate, not ignored under the
cfg}`), and a name-set difference would be empty for every tree, every
night, reporting green having run nothing. `scripts/nightly-only-
selection.py`'s header carries the verified listing output.

**An empty set is legitimate and is still not accepted blindly.** A tree
with no markers has nothing to run; two listings built the same way — the
flag not reaching the second build, a misspelt cfg — produce the same
empty set and would zero the lane permanently. The script separates them
from the SOURCE, the way the interval one does: no marker anywhere under
`crates/` proves the empty case, markers present with an empty difference
is a broken rig and fails.

### 2026-09-03 — `corrupt input (release profile)` demoted to the nightly

S-TCOST unit C1, Ev's approval in chat the same day. The job moved out of
`ci.yml` into `nightly.yml` verbatim — its steps, its non-empty-selection
count guard and its five `... ok` name greps plus the two suite-header
greps — and now runs ungated on any night main moved, rather than on every
code-tier PR run whose closure holds `topo`.

**Argued against §*What is NOT sampled, and the rule*, per row**, which is
what that section demands of a new entry rather than inheriting another
job's licence. The three profile-independent-but-release-named rows assert
on the body the operators produce, and a wrong body persists. `review_d18`'s
two `cfg(not(debug_assertions))` hammer rows run in NO other lane, which is
the case that looks like an absence detector and is not one: what they
detect — a row-4 `unreachable!` becoming input-reachable — is a property of
the tree's code and persists exactly as the rows above do. The genuine
absence risk, those rows silently ceasing to be selected, is what the count
guard and the name greps are for, and they moved WITH the job, so the
detector kept its cadence relative to the rows it guards. The full argument
is at the job, which is where the rule says it belongs.

**Billed minutes: −2 per code-tier PR run whose closure holds `topo`.**
The subtrahend is the job's own line in the reference table at the top of
this document — 1.37 min wall, **2 billed** — and `topo` is in the closure
of 89 of the last 128 first-parent merges. Against that, **~2 billed
minutes a night**: the nightly pays the same rounded-up minute, once, on
days main moved.

The *after* is read from this unit's own PR run, `33721373132` (16 jobs,
default lane drawn): `corrupt input (release profile)` **is not among
them**, which is what −2 means here — the job is gone from ci.yml rather
than shortened, so there is no new duration to read and the delta is the
whole of its old line. What that run cannot re-take is the 1.37/2 itself,
because the job no longer runs there; per the F6 addendum's rule, that
figure is true as of the audit that measured it and the nightly is now
where a fresh reading of it comes from.

**What the demotion gives up, and what it does not.** It gives up
attribution: a break lands on the night's merges rather than on the PR that
caused it. Two handles remain — `nightly.yml`'s `ref` dispatch input
re-measures any commit, and `local-scripts/ci-local.sh` still runs the row
on every local gate, still scoped by `RUN_TOPO_RELEASE`. That local
consumer is why the filter key SURVIVES the demotion: `ci.yml`'s `filter`
job publishes no `run_topo_release` output any more (an output nothing reads
is how a reader concludes a job still exists), but deleting the key would
have promoted a scoped local row to unconditional, which is the opposite of
what this decided.

### `ready_for_review` is load-bearing

The default `pull_request` type set is `[opened, synchronize,
reopened]`. Undrafting a PR pushes no commit, so a draft skip without
this type would leave the skipped run as the PR's only run and every
gate reporting green having executed nothing. A draft skip without it is
not a saving, it is a hole.

### Ranked next, unchanged by this

1. **F1's paired change** — render lanes off PR runs *and*
   `render-hosted.sh` defaulting to dispatch. 12 billed minutes, and the
   largest single item left on the PR side.
2. ~~**`k-lint (gate)`'s cache**~~ — **SUPERSEDED 2026-08-22**, and the
   entry is kept rather than deleted because the reason it was wrong is
   the useful part. It read a 10-minute job as a caching problem; the
   job is five feature unifications that share almost no artifacts, and
   no cache configuration collapses five compilations into one. Sampling
   one of the five per run does, and did (~10 → ~2-3). The full argument,
   the per-row soundness audit it required, and the piece of the cache
   lever that DID land are in the block quote inside F6 above. The
   original entry read:

   > `k-lint` builds the same five excluded roots behind the same
   > unconfigured `rust-cache`, for clippy, tests and a release build
   > rather than a doc pass, and bills 10. Same one-input fix, unmeasured
   > there.

   Sampling the rustdoc gate — the entry this replaces — is off the
   table for now, and F6 was why: the gate was back inside 2 billed
   minutes without giving up a root. **That premise is false as of
   2026-08-31, and the sentence is left standing as the reasoning of its
   date rather than quietly rewritten**: measured warm against warm, the
   job bills 3 at the merge base (job growth unrelated to the gate) and
   4 with the D180/D301 widening's third pass. F6's addendum above
   carries both readings. What needs re-reading is the conclusion, not
   the argument.
   Sampling it *would* be sound (a broken intra-doc link persists in the
   tree, so a later draw finds it) but it is the wrong tool — the roots
   are independent, so sampling them buys latency proportionally rather
   than exploiting near-certain agreement the way eps does. If the
   widening does cost the minute back, this is the trade to re-open
   first.
3. **A scheduled full run on main** — still owed from F3, and now owed
   more: with the push run trimmed and the PR run sampled, no single
   tree is gated at every point by hosted CI. Deliberately not bundled
   here (Ev: "the PRs will get it"). **Unchanged by the nightly lane**,
   which is a different proposal — see *What did not land* above.

**New, and ranked from here (2026-08-22).**

4. **The opt-level verdict is unverified on the box it is about, and the
   lane that settles it is now in the nightly.** ci.yml's OPT LEVEL note
   turns on `r`, the opt-0/opt-2 execution ratio, quoted at 6.46 (default
   lane) and 7.08 (interval) from a developer's box. A census measured the
   same ratio at **4.95 / 4.99** on a 4-core AVX-512 guest — about 30%
   lower, which would turn that note's "~2x and ~3x margins" into 0.94x
   and 0.91x, i.e. opt-0 winning outright. **That is not actionable as a
   flip**: a ratio does not transfer between machines and the census box
   is not CI's 2-vCPU runner. It is actionable as *the number has never
   been measured where CI runs*. `nightly.yml`'s `opt-level calibration`
   job measures it: the arm at the tree's own level read free from recent
   gate runs' step durations, the other levels measured deliberately (opt-2
   and opt-0 as of the 2026-08-25 flip), weekly plus a >20%
   drift trigger, verdict by direct comparison (`a2 + E2 < a0 + E0`) with
   no model. Reporting only. The history is
   `docs/perf-data/opt-level/`. **Read the first few samples before
   quoting either figure again** — including the ones in this document.

   > **THREE ARMS SINCE 2026-08-25**, and the added one is `opt-level = 1`,
   > which nothing in this repository had ever measured, proposed or
   > rejected. Every artifact above compares 0 against 2 and stops — but
   > `a + E` is minimised over a knob with four settings, the two arms sit
   > at opposite extremes of *both* terms, and the build penalty opt-2
   > swallows (`a2 - a0` = 499 s in the 2026-08-25 sample) is more than
   > twice the margin it wins by (220 s). A three-arm sweep on a 4-core
   > AVX-512 guest — the same class as the census box, *not* the runner —
   > found opt-1 within 3% of opt-2's execution for 58% of its build
   > penalty, winning outright at 367 s against opt-0's 432 s and opt-2's
   > 485 s. That is why arm C is wired up; it is **not** evidence about
   > CI, and only arm C's own samples can be. **The doubled cost is the
   > `opt-level` row in the budget table above** — the second measured arm
   > is the whole of it, since arm A stays free.
   >
   > Wiring it up also uncovered that the same-suite cross-check those
   > samples advertise had never run: nextest colourises on a hosted
   > runner and the SGR escapes around the count defeated the extraction,
   > which is why both schema-1 samples read `"tests": "unknown"`. The
   > measured arms now pass `--color never`. The free arm still reports
   > `n/a` — the jobs API gives durations, not test counts — so what the
   > check compares is the measured arms against each other.
   >
   > **AND THEN THE TREE MOVED (Ev, 2026-08-25): `ci.yml`'s two archive
   > jobs are at `opt-level = 1`.** Made on the sweep above, i.e. on
   > evidence from a box this lane explicitly distrusts, before a single
   > runner sample of opt-1 existed — deliberately, because *the fastest
   > way to get runner data on opt-1 is to run the gate on opt-1*. Every
   > PR now produces a real opt-1 archive step and a real opt-1 test row,
   > and the lane reads exactly those durations for free.
   >
   > **The flip forced a redesign of the lane, and the reason is worth
   > keeping.** The free arm is free only because the gate already runs
   > it, so it is whatever level the gate is set to. The letters had the
   > level welded in (`arm_a.a2`); flipping the tree without schema 3
   > would have left the free read filling `a2`/`E2` with opt-1 durations
   > while a measured arm took opt-1 again — one sample carrying opt-1
   > twice, once mislabelled, verdict computed off it, nothing red.
   > Arms are keyed by level now, `tree_opt_level` is recorded, and a
   > guard step refuses to run unless the free and measured levels
   > partition {0,1,2}. The measured arms are opt-0 and opt-2, so
   > reverting stays a measured decision.
   >
   > **Read the first post-flip samples with suspicion**: the knob change
   > rotates ci.yml's rust-cache key, and the lane's opt-2 arm builds
   > against a brand-new key of its own. Both buy one cold rebuild.
   >
   > **The billed-minutes effect on the PR side is not yet measured** and
   > deliberately not estimated here — the gate's own runs are now the
   > measurement, and the budget table above still carries the opt-2
   > figures until there is something real to replace them with.
5. **`k-lint`'s cache, at its new size.** The lever is not wrong, just
   proportionally smaller: it now applies to whichever single unification
   a run drew. Unmeasured, and worth less than it was.
6. **F4's sccache reading** — TAKEN (2026-09-03, PR 1648), and it is a
   negative: sccache refuses `--crate-type bin`, which is every test
   binary in the archive and 82 % of the job's compile time, so the
   workspace-crate hit the note priced was never available. What the
   same runs turned up instead is that `Swatinem/rust-cache` restores
   **nothing** on most build jobs — that is the open lever now, and F4
   states it.

## 2026-08-28 — asking for a point instead of drawing one

The draw above is a **default, not a lock**. Two entry points let a
person name the point a run gates; both land in the same place —
`scripts/ci-filter.py`'s LANE / EPS / KLINT_ROW output lines — so a
requested run is byte-for-byte the run that point would have been, and
no job condition, matrix or cache key learns a new word.

* **`workflow_dispatch` on ci.yml.** Inputs `lane`, `eps`, `klint`, each
  defaulting to `sample` (draw it as usual), plus `scope` — classify
  against the default branch, or run everything unscoped.
* **A `CI-Config:` trailer in the head commit's message**, e.g.
  `CI-Config: eps=1e-12 klint=dev-probe`, read on ordinary
  `pull_request` runs. The HEAD commit's and only that one, so a request
  lasts exactly one push — the next commit, a merge of main into the branch
  included, samples again unless it carries the trailer too. A request that
  persisted over a branch would gate commits nobody wrote it for.

**Why both, when either alone answers the ask.** They fail at opposite
ends, and the two failures are the two things one actually wants to do:

| | dispatch | trailer |
|---|---|---|
| needs a commit | no | yes |
| can target an already-landed tree | yes | no (no history rewriting here) |
| reports on the pull request | no — checks belong to the run | yes |
| which point gated is recoverable from the commit | no | yes |

That last row is the property the sampling was built around, and a
dispatch necessarily suspends it. So the run says so itself: the `filter`
job prints `CONFIG_SOURCE` — `sampled` / `requested` / `commit-trailer`,
per dimension — in a step with no `if:`, and publishes it as a job
output. A run at a chosen point that looked exactly like a run at the
drawn one would reintroduce, by hand, the silent-coverage failure the
`klint_row` lesson is about.

**Precedence** is invocation over trailer over draw, per dimension, so
`--config eps=1e-12` means "1e-12, and surprise me twice".

**Shown to fire, hosted, before it merged.** The DRAWN half: the
`change filter` job of run
[`33191437807`](https://github.com/evgunter/cad/actions/runs/33191437807)
— the feature's own PR — printed `lane=default eps=1e-6
klint_row=release-default` / `source: lane:sampled eps:sampled
klint:sampled`, which is also what proves the hosted-only line in that
step (reading the PR HEAD commit's message out of a merge-ref checkout)
works at all. The REQUESTED half: the commit carrying this paragraph
asks in its own message for `eps=1e-12 klint=dev-probe`, so its run
gates a second point of the matrix and its `source:` line names the
trailer for exactly those two dimensions.

**A malformed request is a red step, not a fallback to the draw** — an
unknown key, an unknown value, a repeated dimension. This is the one
place in `ci-filter.py` that does not fail into more work, and the
asymmetry is deliberate: every other failure there is an inability to
classify, where running everything is the safe answer, while this one is
an input error whose author is standing there reading the result. Failing
open would hand them a green run over a configuration they did not ask
for, which is exactly the question they were asking. `eps=all` is refused
for a duller reason: it is the LOCAL half's word for "loop the rows",
while the hosted rows interpolate the value into `CAD_TOLERANCE_EPS`,
where it is a parse error by design.

**What this does and does not do to item 3 of the ranked list** (*a
scheduled full run on main*). It does not close it — nothing here fires
on its own, and an unrun gate is not a gate. What it removes is the
**helplessness**: "no single tree is gated at every point by hosted CI"
was true with no way to fix it for a particular tree, and a dispatch at
`scope: all` against main now gates a landed tree at a named point on
demand, one point per run. The scheduled run, if it ever lands, is that
without someone having to decide to press it.

**Cost, unchanged and worth restating**: a dispatch bills what a code-tier
PR run bills (~40 above) and buys a *second* gate over a tree the ordinary
run already gated at one point — and `lane: both` or `klint: all` put back
rows the sampling removed. This is the escape hatch, not the new normal.
The render lanes are the one thing a dispatch skips — no lane reads any
sampled dimension, they re-baseline against a branch, and `render.yml` has
its own dispatch for when frames are what is wanted.

## Observed flake: the probe-suite census's own SELFTEST (2026-08-29)

Recorded because it cost a lane a red run over content that was green,
and because the gate's own comments say this failure mode was already
seen once and believed structurally fixed.

**What happened.** `probe-suite-census.sh --selftest` failed on the
hosted runner inside the "CI half parity + gate wiring" job, reporting
`SELFTEST FAILED: the gate FAILED on a clean fixture with a long
ci.yml` together with `printf: write error: Broken pipe` and a
downstream complaint that `span_meter_dim_twins` is rostered with no
censused file — a suite the failing branch never touched.

**Why it is a flake and not a finding about the tree.** Three
independent pieces:

- the SAME census inputs passed on the immediately preceding run
  (33245525736 → 33265515891 green on the gate-wiring job; 33266619334
  red), and the only tree delta between them was the body of one
  unrelated test file;
- in the very run that went red, the REAL gate — the "probe-suite
  census (crates + floor)" step in `discipline (evaluation-code)`,
  which runs against the actual tree rather than a synthetic fixture —
  **passed**;
- the selftest passes locally on the same tree.

**Where the race is.** `selftest_hosted_half_is_large` exists precisely
because a `grep -q` that matches near the top of a long `ci.yml` leaves
the upstream filter writing into a closed pipe, which `pipefail` then
reports as a failed pipeline. The function pads a fixture past the
match to make that deterministic. The evidence above says the
structural fix does not cover every path: the broken-pipe message in
this failure comes from the census's own `printf` at the roster-listing
site, not from the padded `grep`.

**What is owed.** Not this note's author's to fix — the gate is the
disciplines lane's. What a fix needs is for every producer in that
script's pipelines to tolerate a closed reader (or for the readers to
drain), rather than one more `grep` being padded around. Until then a
red on this step alone, with the real census step green in the same
run, is a re-run and not a diagnosis.

## 2026-09-03 — the python suite becomes seed-keyed

S-TCOST unit C3, Ev's approval in chat the same day. `python suite
(wheel + guide + north-star)` was gated on `pncad-py` being in the
dependent CLOSURE — the wheel compiles that crate's whole dependency
graph, so the condition read as "something the wheel compiles moved".
`pncad-py` sits under `pncad`, which re-exports the entire kernel, so
that is true of nearly every kernel change: the gate selected almost
nothing, and what it bought on each of those runs was a SECOND compile
of the kernel under the non-default `python` feature.

It is now keyed on the change filter's SEEDS — the members whose OWN
files moved — intersecting `{pncad-py, pncad, editor-core}`, exactly the
shape and exactly the argument of the viewer toolkit axis
(`RUN_VIEWER_TOOLKIT`, Ev's 2026-08-27 ruling), one crate over.

**Billed minutes: −2 on every code-tier PR run whose seeds miss that
set.** Read from this unit's own run, `33722922975` (head `13f8a2fb`,
interval lane drawn, 21 jobs, ~62 billed): the job ran 120 s, i.e.
**2 billed**, which is also what the audit table at the top of this
document recorded (1.58 min wall, 2 billed) — so this is the row's cost
re-taken rather than inherited. Against that, ~2 billed minutes a night
for `nightly.yml`'s `python suite (ungated re-take)`. The run this
figure comes from is itself a tier-`all` diff, so the axis was TRUE
there; the saving is on the kernel-change population it is now false
for, which is the majority.

**Argued against §*What is NOT sampled, and the rule*, per this row.**
What the suite detects persists: a broken `.pyi` signature, a guide
script that stops running, a north-star assertion that stops holding.
It is not a detector of absence — the suite is DISCOVERED
(`unittest discover`), not listed, so a test module that vanishes is
not something this row reports on at any cadence. The one hole the
seeds open is a kernel change that moves a NUMBER the `.py` assertions
pin while touching no seed; that is the analogue of the viewer axis's
toolkit-dependency drift and gets the identical answer, the ungated
nightly re-take. A change that BREAKS the re-exported Rust API is not
in that hole: it reds the offending crate's ordinary closure rows on
the same PR.

**AND THE RE-TAKE IS GUARDED AGAINST RUNNING NOTHING.** `unittest
discover` prints `Ran 0 tests ... OK` and exits 0 over a directory whose
modules stop matching, so an ungated nightly lane could report green
having executed nothing — which is the ABSENCE case this section forbids
demoting anything into. The count is read back, required to be non-zero
and echoed, at all three sites that run the suite (both hosted jobs and
`crates/pncad-py/run-python-tests.sh`); the three copies and why no
shared runner exists are filed at
`work/issues/python-suite-zero-test-guard-three-copies.md`.

**RECORDED, NEVER SILENT — and it needed a different seat from the
viewer's.** The viewer axis prints its verdict inside `fmt`, beside the
rows it gates. This axis cannot: when it is false the whole JOB is
skipped, and a skipped job runs no step at all, so there is no seat
inside it from which to speak. The verdict is a step of the `filter`
job, which computed the value and carries no `if:` — `python suite -
the filter's verdict`, printing the seeds it was decided from.

## 2026-09-03 — the build job's cache was in a scope no branch could read

F4's third finding — `No cache found` on five of seven build jobs and on
the control — is the lever it said was worth pulling next. This is the
reading of it, and the change that follows.

### Method

Read off hosted logs only, on `main` as it stood with the sccache rig
inert (`vars.SCCACHE == '1'`, unset), so no sccache confound sits inside
any pair below.

* **Keys and restore results**: the `Cache Configuration` block and the
  line after `... Restoring cache ...` in each `build + archive` job's log.
* **The key population**: ten distinct PR branches, each one's FIRST CI
  run (oldest run for that `head_branch`), every non-skipped build job in it.
* **The miss rate**: every non-skipped build job in the 60 most recent
  completed `pull_request` runs — 49 with a readable restore line.
* **Durations**: the `build test binaries + archive` step's own
  `started_at`/`completed_at` from the jobs API, never the job wall.
* **Rates**: run counts from the workflow-runs API over the window each
  figure names.

### The keys: one per lane, identical on every branch

| branch | first run | job | key | restore |
|---|---|---|---|---|
| `lib/b-validate4` | 33745815456 | default | `v0-rust-build-Linux-x64-6f07d2f1-66da18f8` | No cache found |
| `lib/b-format` | 33739144114 | default | same | No cache found |
| `lib/b-resolve` | 33735881765 | default | same | No cache found |
| `tcost/7-geom-brep-test-helpers` | 33716129026 | default | same | No cache found |
| `verbs/rimcap-1` | 33743842534 | interval | `v0-rust-interval-build-interval-Linux-x64-6f07d2f1-66da18f8` | No cache found |
| `verbs/1031b-winding` | 33742846713 | interval | same | No cache found |
| `tcost/b2-dedup-remaining` | 33739184479 | interval | same | No cache found |
| `lib/b-expr-read` | 33733876171 | interval | same | No cache found |
| `tcost/8-geom-brep-helpers-2` | 33728831063 | interval | same | No cache found |
| `tcost/k1-flux-budget-exit` | 33721985045 | interval | same | No cache found |

Ten branches, ten misses, **two distinct key strings**. That is the whole
diagnosis: the key does not vary, so the miss is not key rotation. It is
**scope**. A GitHub Actions cache entry is readable from the ref that wrote
it plus the default branch; `build` and `build-interval` carry
`github.event_name != 'push'` (F3), so no run on `main` had ever written
an entry under either key, and a branch could only ever restore its own.

`6f07d2f1` is the rust-environment hash and `66da18f8` the lockfile hash.
Neither **eps** nor the **tier** appears: eps is read at runtime, and the
tier changes `cargo_scope` — which packages are archived — not the env
block and not the lockfiles. C4's trial runs carry `61bb9c0c` in the env
slot instead, which is `RUSTC_WRAPPER=sccache` being hashed, exactly as
F4's note says.

**Does `main` ever save a build-job cache?** No — not in any workflow.
`nightly.yml` builds no archive by its own header's rule, and its
rust-cache entries are all under `nightly-*`, `topo-release` and
`opt-level-calibration-*` keys. What main DOES hold is
`nextest-0.9.140-Linux-X64` (11 MB, `actions/cache`, written by a
nightly), and `lib/b-validate4`'s first-ever run restored it — which is
the in-repo proof that a main-scoped entry is readable from a fresh
branch's first job, and that a constantly-restored entry survives this
repo's cache churn.

### The size of it

* **The population: 53 non-skipped build jobs** in the 60 most recent
  completed `pull_request` runs, spanning 3.20 h — **16.6 an hour**. 49 of
  the 53 had a readable restore line (the other 4 were cancelled before
  the step printed one).
* **Miss rate: 40 of those 49 (82 %)** — 14/18 default, 26/31 interval.
* **Cold minus warm, matched within one branch, same lane, same scope**
  (`tcost/c2-rustdoc-roots-nightly`, `--workspace`): default **820 s**
  cold → **639 s** and **603 s** warm; interval **840 s** cold → **677 s**
  and **606 s** warm. About **200 s** either way. The population medians
  agree: default `--workspace` 775 s miss (n=6) against 631 s hit (n=1),
  interval 830 s miss (n=7) against 634 s hit (n=2).
* **So the bill**: 16.6 jobs/h x 0.82 x 200 s = 2 722 s/h = **~45 billed
  minutes an hour** spent recompiling ~225 dependency crates.
* **The entry is 276 MB** and every missing job saves one, which is
  ~4.6 GB an hour of writes against a 10 GB repository budget — the same
  churn that evicted C4's sccache entries inside the hour. Every one of
  those saves is readable by exactly one branch.

### The lever: write the entry where every branch can read it

`cache-prime` and `cache-prime-interval`, on `push` to main (and on
`workflow_dispatch`), each restoring under the same `shared-key` its
build job now uses and running `cargo test --no-run --workspace` **only
when `cache-hit` is not an exact match**. Both build jobs move from the
job-id default key to `shared-key: build-default` / `build-interval`,
which is what lets a second job spell the same key; the values stay
distinct, so the two feature graphs stay as separate as before.
`scripts/check-cache-prime-parity.py`, in the `discipline` job and in the
local half, holds the coupling: the primer and the job it primes must keep
identical `env:` blocks and one shared key each, or the build is red
rather than quietly cold.

**What it costs.** Two denominators, kept apart. Of the **2962 commits**
main took in 14 days, **5** touched `Cargo.lock`, any crate's
`Cargo.toml`, `.cargo/config.toml` or `rust-toolchain.toml` — the only
files that rotate the key — so a rotation is a ~3-day event. Separately,
main received **3.4 PUSHES an hour** (200 `push` runs of ci.yml over
59.3 h), and it is pushes, not commits, that fire these jobs: 3.4 x 2
jobs x one billed minute (a checkout plus a 14 s restore) = **~7 billed
minutes an hour**, against the ~45 above.

**A rotation costs at least one full dependency build per lane, and can
cost several.** This workflow's `cancel-in-progress` cancels the running
push run when main moves again; at 3.4 pushes an hour a ~13-minute
priming build is often cancelled part-way, `cache-on-failure` is off (see
below), and the repair restarts on the next push. Every branch is cold
until one completes, so "one build per rotation" is a floor and the
lower-bound figure, not an expectation.

**A second effect, not the one we went looking for.** rust-cache does not
re-save on an exact hit ("Cache up-to-date." — observed in every warm job
read here). Once a branch hits main's entry it writes nothing, so the
~4.6 GB/h of near-identical per-branch saves goes away with the misses.

### Declined, with the reason

* **A scheduled or per-merge full build on main.** A full build per main
  push is 3.4 × 2 × ~800 s ≈ 90 billed minutes an hour, more than the 45
  it would save. The `cache-hit` guard is what makes the same idea pay.
* **A nightly-only save.** One save a night cannot repair a rotation for
  up to 24 h, and this repo rotates the key every ~3 days. `nightly.yml`'s
  header also forbids building the gate's archive there.
* **`cache-on-failure: true`.** This workflow cancels an in-progress run
  when main moves again, so a rotation's prime can be cancelled part-way;
  saving that partial target directory would store a partial dependency
  set under the EXACT key, after which every later push would see
  `cache-hit` and skip the build that would repair it. Off, a cancelled
  prime writes nothing and the next push retries. On the branch side it
  is moot after this change: branches that hit exactly no longer save.
* **One entry for both lanes.** Measured, and it would work: two
  `--workspace` cold runs, one per lane, compile the **same 172 registry
  crates** (`interval-transcendentals` is in both, because `geom-core`
  carries it as a dev-dependency). The `interval` feature adds one local
  path crate and changes feature flags only on workspace crates, which
  rust-cache strips before saving. Not taken: `build-interval`'s CACHE
  SEPARATION note argues the split on the graphs, one job-night is the
  whole saving, and this measurement is recorded here for whoever wants
  to re-open it.

### What this cannot show yet, stated rather than implied

**There is no after-reading from a PR run, and there cannot be.** The
priming jobs run on `push` to main and on dispatch; a branch's first build
job can only restore main's entry once this is merged and one push to main
has primed it. What the PR run does show is the two build jobs computing
the new `shared-key` form. What it does NOT show is the primer computing
the *same string* — the one coupling that can fail silently — because the
implementing lane's token could not dispatch this workflow (HTTP 403) and
`push` runs only on main. That coupling is therefore held by
`scripts/check-cache-prime-parity.py` rather than by a measurement: it
fails the `discipline` job if the two jobs' `env:` blocks or their
`shared-key`s ever differ, which is the whole of what a lane can control
from a branch. **The first reading owed, and its shape: the first PR
opened after this merges, first run, `build + archive`'s restore line and
the `build test binaries + archive` duration**, against the 820 s / 840 s
`--workspace` cold figures above at the same tier. If that line still says
`No cache found`, compare the two `Cache Key:` strings — this job's and
the primer's on the merge commit's push run — before anything else.

**The residual miss population is sampling's, not scope's.** A branch
draws its lane per run, so a branch that pushes twice is cold in the lane
it has not yet built — which is why the interval miss rate (26/31) sits
above the default one (14/18): the interval build is the rarer draw, so
its branch-own entry is the one that has usually been evicted. After this
change both lanes are primed on main, so the draw stops mattering for the
first run; what remains is the branch that rotates the key itself.

Two more things one run cannot settle: whether main's entry survives
eviction over days (the mechanism says it should, since every restore
refreshes it and the competing 4.6 GB/h of per-branch saves is what this
removes), and whether the ±25 % spread B1 recorded across `tier=all`
samples swamps a single pair — read several.

**And the first run after this lands is cold by construction**, in both
lanes: `shared-key` changes the key string, so the old entries are
unreachable. That run is not the verdict, the same trap the OPT LEVEL
note in ci.yml warns about.

## 2026-09-04 — the re-take on the public 4-vCPU runner (CIW unit 8)

**This document's opening premise is dead.** *"The Actions allowance was
being consumed faster than the work justified"* was true of a private
repository. `evgunter/cad` went public on 2026-09-03 (`5cc16e81` …
`483212ef`; the repository record read `visibility: public`,
`private: false` on 2026-09-04), standard-runner minutes are not billed,
and the runner is **4 vCPU / 16 GB** — read first-hand off run
**33830873453**, job **100893490483**, whose `runner + linker
provenance` step prints `nproc` = **4** and `free -g` total **15**.

**Every timing above this section predates that runner.** They are an
ordering of costs, not a budget. What follows re-takes the ones a
decision now rests on. The population is `pull_request` runs created
between `2026-09-03T15:20Z` and `2026-09-04T05:47Z` with
`status=completed` **and `conclusion` in {`success`, `failure`} —
cancelled runs excluded**, since a cancelled run's job set is a
truncation rather than a sample: **220 runs, 149 of them code-tier**,
read from the jobs API. (Keeping the cancelled ones in gives 268 / 195
and moves every duration; the exclusion is part of the frame, not a
filter applied afterwards.)

| what | 2 vCPU (above) | 4 vCPU (this section) |
|---|---|---|
| code-tier run, created → last job end | 13.75 min (critical path) | **7.4 min** median (442 s); **8.03 min** (482 s) at `tier=all`, n=66 |
| `build + archive (default)`, `--workspace` | 820 s cold / 603–639 s warm | **336 s** median (n=23) |
| `build + archive (interval)`, `--workspace` | 840 s cold / 606–677 s warm | **388 s** median (n=43) |
| a code-tier run's total job time | ~87 billed min, then ~62, then ~40 derived | **24.4 job-minutes** median (1462 job-s over 15 live jobs) |

Two changes are inside that difference — the core count and the cache
priming landed the same day — and this reading does not separate them.
It is what a run costs today, which is what the decisions need.

**The billing model at the top of this file no longer applies.** Per-job
round-up is not a cost when minutes are not billed; the currency is
**wall clock** (latency to a verdict) and **runner load** (job-minutes an
hour, against concurrency). Quote those, not billed minutes, in any new
argument.

### What this changes for F3, and what it does not

The re-costing of F3 itself, its options and a recommendation are in
`work/ciw/f3-recosting-on-a-public-repo` and go to Ev on an `[ev]` PR.
Three results from it belong here, because they correct or complete
statements this document makes:

1. **F3's push-run saving is now 24.4 job-minutes per code-tier merge
   and zero dollars.** At the measured merge rate (200 push runs over
   45.66 h = 4.36/h, of which 90 are code-tier = 1.97/h) restoring the
   full set would cost **+48 job-minutes an hour**, or **+0.80 mean
   concurrent jobs** against a measured mean of 4.3, a p90 of 12 and a
   peak of 36 from PR runs alone. Queue delay today is 3 s median, 25 s
   at p99: there is no queue for it to join.
2. **"main's push runs classify docs-tier" is false, and this document
   is not where that claim lives, but its readers act on it.** 90 of 200
   push runs (45 %) ran `renders`, which requires `RUN_K_LINT=true`,
   which `scripts/ci-filter.py:1730` sets for every tier but `docs`;
   `RUN_BUILD` follows the same rule at `:1713`. The test rows are
   skipped on **100 %** of push runs by F3's `github.event_name !=
   'push'` guard, not by the tier.
3. **A restored push run would be cancelled more often than not.**
   Median gap between pushes on `main` is **308 s** and only **36 %** of
   code-tier pushes have ≥442 s before the next one. It is not
   speculative: **51 of the 90 code-tier push runs (57 %) are already
   cancelled today**, at a job set whose median is 259 s, against 15 %
   of docs-tier pushes at a 40 s median. The longer the run, the more of
   them die, and a 442 s run sits above both readings. Restoring jobs
   without also stopping the cancellation buys a gate that does not
   finish — and stopping it is **three interacting mechanisms**, not one
   line: `render.yml:268–275`'s own gate-mode group keyed on the
   caller's ref (which *starts* firing once the run-level group goes
   per-SHA), the `cache-on-failure: false` argument at 1830–1840 above,
   which is argued *from* push runs being cancelled, and `renders`'
   `push_to` commit to `main` at 4203, which assumes serialisation.
   Priced in `work/ciw/f3-recosting-on-a-public-repo`, which does not
   price the design pass and says so.

### The scheduled full run: still declined, and now for a better reason

Priced at today's rates it is **9.8 job-hours a day hourly** or 24.4
job-minutes a day nightly — *cheaper* than a per-merge run, since it does
not scale with the merge rate. It stays declined anyway, and the price
change is not why: measured on the one instance in evidence, the next
PR's merge-ref run was created **11 m 41 s** after the composing merge
and went red **17 m 29 s** after it (run `33788618577`, job
`100761051102`, +348 s into its own run). So a scheduled run buys a
slower copy of a discovery that already happens, and names a window of
~4.4 merges instead of one. What the residue costs is attribution, and a
scheduled run does not supply it.

**For the same reason a per-merge run is not a slower copy: it is a
faster one.** At the same 348 s in-run offset a push run on that merge
reds at `18:01:30Z` — **11 m 41 s before** the PR run did, because it
starts 11 m 41 s earlier. Any comparison that puts the PR run first is
subtracting a run *duration* from a run *creation* time.

### The cache section's first reading owed, taken

That section asks for "the first PR opened after this merges, **first
run**, `build + archive`'s restore line", and the first-run half is the
half that carries it. Run **33827576986** is the **first-ever** `ci.yml`
run on branch `m10/hotfix-tag-inventory`; its job **100883473006**
compiled the workspace crates and a handful of registry crates,
`Finished 'test' profile … in 54.22s`, and its post step printed `Cache
up-to-date.` — rust-cache's exact-hit, nothing-to-save line. A cold
build of that scope recompiles ~225 registry crates, and this branch had
saved nothing of its own, so the entry came from `main`'s scope.
**The primer works and a branch now inherits `main`'s entry.**

(Job **100893490483** shows the key and the hit —
`Cache hit for: v0-rust-build-default-Linux-x64-fa41882e-fd5fb1c1`,
263 MB, `full match: true` — but it is that branch's fourth run and
could have restored its own save, so it is not the evidence for
inheritance.)

`work/tcost/rust-cache-never-restores-across-branches` still says no PR
can inherit one; that item is S-TCOST's to update.
