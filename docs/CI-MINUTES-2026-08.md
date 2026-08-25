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
agent tool, not a CI tuning knob, and belongs to Evan.

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

**Landed** (Evan authorised, 2026-08-20; commit `0768882`): keep the `push: main` trigger but
reduce it to `filter` + `rebuild-latency` + `renders`, skipping build,
test, clippy and k-lint. Preserves both write paths; drops ~65 of the
87 minutes per merged code PR.

**What it gives up:** the landed main commit is then never itself
tested. PR runs test the merge-ref, so when main moves between a PR's
last run and its merge — frequent at this repo's merge rate — that
exact combination went untested.

**The scheduled full run on main that would have paired with this is
DECLINED** (Evan, 2026-08-22). The next PR's merge-ref is main plus
that branch, so it tests the landed tree anyway; a scheduled run buys
a second discovery of the same fact and costs a full gate per period
whether or not anything landed. The residue is accepted, not
outstanding: **a semantic conflict between two independently-green PRs
surfaces on the next, innocent PR** rather than at the merge that
caused it, and the person who gets the red did not write the code that
caused it. The reading that goes with that is in ci.yml's header.

### F4 — sccache: the local revert does not transfer to CI — ON TRIAL

`docs/LOCAL-BUILD-PERF.md:109` reverted sccache locally on a real
measurement: cold build 156 s → 96 s at a 99.4% hit rate, given up
because sccache and incremental compilation are mutually exclusive and
going non-incremental cost **5–7x on the edit-rebuild loop** (91 s vs
18 s on geom-core). `local-scripts/gate.sh:37` keeps sccache on the
local gate runner for exactly the cold-build case.

**That objection is already moot on the runner.** `Swatinem/rust-cache`
sets `CARGO_INCREMENTAL=0` itself, on every job that uses it — which
is every compiling job here. **CI has already paid sccache's only
documented cost, unconditionally, and gets nothing back for it.**

That does not make sccache a win; it makes the question sharp. Since
rust-cache already caches the ~225 dependency crates, sccache's unique
contribution in CI is exactly the **workspace** crates rust-cache
evicts — which ci.yml:528 and render.yml:718 both call out, and which
at opt-2 dominate the two 11–12 minute build jobs. The 156 → 96 s
local figure does **not** measure that and must not be quoted for it.

**The rig is in the tree and is ON.**
`.github/actions/install-sccache` installs the pinned 0.16.0 prebuilt
(the version `local-scripts/gate.sh` already runs locally) and restores
a per-lane object cache; the two build jobs carry `RUSTC_WRAPPER` and a
`sccache --show-stats` step. The kill switch is the repo variable **`SCCACHE`** set to `"0"` — unset
means enabled, so the trial runs without a variable needing to exist.

Verified locally before landing, on the exact artifacts CI will use:
the release URL resolves (HTTP 200), the tarball's layout matches the
extract path the action uses, the binary runs (`sccache 0.16.0`), and
wrapping `rustc` on a throwaway crate gives a **cache miss cold, then a
cache hit after `cargo clean`** — which is precisely CI's situation,
since rust-cache restores the deps but not the workspace crates.
That proves the mechanism, not the size of the win.

**Tracked in #853, to be read in a few days' time.** *Discard the first run after
this landed* — `RUSTC_WRAPPER` is RUST*-prefixed, so rust-cache hashes
it and the flip buys one cold rebuild (the same trap the OPT LEVEL
note on the build job warns about). Then compare the `build test
binaries + archive` step duration against the pre-sccache baseline in
the table above — **11.90 min (interval) and 11.03 min (default)** —
and read `sccache --show-stats`: **hits on dependency crates prove
nothing**, rust-cache already serves those. Only workspace-crate hits
on a warm run justify keeping it. To revert, set `SCCACHE=0`; to adopt,
delete the variable check.

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
* `#921` — the `fmt` job's `Swatinem/rust-cache` told about all seven of
  the cargo roots the rustdoc gate documents, instead of the one it
  defaults to (F6). The list is not written into `ci.yml`: a step asks
  `scripts/doc-gate.sh --print-roots`, which runs the gate's own
  derivation, so the cache's scope cannot drift from the gate's coverage.
  **−1 billed min.**

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
  mechanism** (Evan: *"no need to make any special attempt to keep
  m4_pr8_corpus visible. just make it a normal test"*) — no filter
  expression, no named row, no doc pointer. `m4_pr6_eps_diff::` never
  needed the loop at all: it re-execs itself per ε, which is the only way
  two ε values can exist in one audit. **−4 billed min.**
* **`k-lint (gate)` samples ONE of its five feature unifications per
  run**, drawn under a salt of its own. See the superseded-cache block in
  F6 above for why this and not a cache, and for the per-row
  absence-detector audit that had to come first. **−7 to −8 billed min**,
  the largest single item in this pass.
* **`watertight` → the nightly** (Evan, explicit). A persistence-detector
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
  F3 left owed and Evan declined**; see *What did not land* below, where
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
| `demoted` | ~11 | TWO workspace builds — the gate-side listing and the `--cfg nightly_suite` one — then ~64 s of tests |
| `watertight` | ~2 | |
| `rebuild latency` | ~2 | its own compile, deliberately not the archive |
| `gate` + `record` | ~2 | |
| `opt-level` | ~2 | arm A only; **+~25-30 one night a week** when arms B and C run |
| **an ordinary night** | **~19** | **~45 on a calibration night** |

**`demoted` is over half of it, and the reason is structural rather than
sloppy**: the selection is a difference between two listings, and a
listing is a build. The gate-side one is therefore taken at **opt-0** —
nothing is executed from it, and the selection reads test NAMES and
`ignored` FLAGS, neither of which an optimisation level can move — which
is ~130 s against the ~430 s the opt-2 build costs (this document's own
F-numbers). The run itself keeps opt-2, because the demotion reasons
written at each test quote their cost *at CI's opt-2 settings* and a
nightly measuring a different profile would be answering a different
question.

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
* **a scheduled full run on main** — DECLINED (Evan, 2026-08-22), not
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

Evan's proposal, and the reason it is a separate section rather than a
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
makes it derivable after the fact.

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

**`local-scripts/ci-local.sh` is now the only lane that runs every point
on one tree**, and is deliberately not sampled: nothing bills it by the
minute. Local is a strict superset of any hosted run.

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
* **opt-level 2 stays.** Re-asked because the verdict rested on
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

**Evan's constraint, and it holds by construction rather than by a
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
   table for now, and F6 is why: the gate is back inside 2 billed
   minutes without giving up a root. Sampling it *would* be sound (a
   broken intra-doc link persists in the tree, so a later draw finds it)
   but it is the wrong tool — the six roots are independent, so sampling
   them buys latency proportionally rather than exploiting near-certain
   agreement the way eps does.
3. **A scheduled full run on main** — still owed from F3, and now owed
   more: with the push run trimmed and the PR run sampled, no single
   tree is gated at every point by hosted CI. Deliberately not bundled
   here (Evan: "the PRs will get it"). **Unchanged by the nightly lane**,
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
   job measures it: arm A (opt-2) read free from recent gate runs' step
   durations, arm B (opt-0) measured deliberately, weekly plus a >20%
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
   > measured arms now pass `--color never`. Arm A still reports `n/a` —
   > the jobs API gives durations, not test counts — so what the check
   > compares is arm B against arm C.
5. **`k-lint`'s cache, at its new size.** The lever is not wrong, just
   proportionally smaller: it now applies to whichever single unification
   a run drew. Unmeasured, and worth less than it was.
6. **F4's sccache reading**, still owed (#853), and now against a
   different baseline: the two build jobs are one build job per run since
   sampling, so a workspace-crate hit is worth half what the F4 note
   priced it at.
