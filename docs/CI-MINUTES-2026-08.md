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
exact combination went untested. Pair the retirement with a
**scheduled full run on main** so integration failures surface in one
cheap run rather than one per merge.

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
collecting; neither was measured here. Still off the critical path:
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
more, and is still unmeasured.

## Where that leaves a code-tier PR

Roughly **87 → 79 billed minutes** on the pull_request side (the four
job merges), plus the post-merge run dropping from ~87 to ~16. For a
PR that lands after N pushes, total spend goes from `87N + 87` to
`79N + 16`.

The PR-side figure is deliberately modest. The three biggest line
items — the two build jobs at 12 each and `k-lint` at 10 — are the
critical path and were not touched; `renders` at 12 is F1, and is a
feature rather than waste. What remains, in rough order of size, is
F3's missing scheduled main run (owed), F5 (policy), F1's paired
change to `render-hosted.sh`, and whatever F4 measures.

## What did not land, and why

* **render lanes off PR runs** — F1. Breaks `render-hosted.sh`'s
  default path. Needs the paired change to that script.
* **merging the default-build test shards** — F2. Costs ~7% run
  latency to save 1 minute.
* **merging `clippy` with `clippy (interval)`** — the two compile
  different feature unifications and share no artifacts, so the merge
  buys only one runner setup (~1 billed min) while serialising ~4
  minutes into a single job. Not worth it.
* **a scheduled full run on main** — the mitigation F3's trim pairs
  with; the next PR adds it.
* **draft-PR skip** — F5. Policy decision.
