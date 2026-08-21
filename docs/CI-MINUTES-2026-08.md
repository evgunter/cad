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

### F3 — post-merge runs on main duplicate the PR run

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

**Proposed shape (needs Evan):** keep the `push: main` trigger but
reduce it to `filter` + `rebuild-latency` + `renders`, skipping build,
test, clippy and k-lint. Preserves both write paths; drops ~65 of the
87 minutes per merged code PR.

**What it gives up:** the landed main commit is then never itself
tested. PR runs test the merge-ref, so when main moves between a PR's
last run and its merge — frequent at this repo's merge rate — that
exact combination went untested. Pair the retirement with a
**scheduled full run on main** so integration failures surface in one
cheap run rather than one per merge.

### F4 — sccache: the local revert does not transfer to CI

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

**Still an experiment, not an adoption.** What it needs: an in-repo
composite installing sccache from the official release (mirroring
`.github/actions/install-nextest`, per the no-third-party-action rule),
`RUSTC_WRAPPER` on the two build jobs only, and a cold run plus two
warm runs compared against main's `build test binaries + archive` step.
Adding `RUSTC_WRAPPER` rotates the rust-cache key, so **run 1 after
the change is cold and is not the verdict** — the same trap the OPT
LEVEL note on the build job already warns about.

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

## What did not land, and why

* **render lanes off PR runs** — F1. Breaks `render-hosted.sh`'s
  default path. Needs the paired change to that script.
* **merging the default-build test shards** — F2. Costs ~7% run
  latency to save 1 minute.
* **retiring the post-merge main run** — F3. Needs a decision on
  losing post-merge integration coverage.
* **sccache** — F4. Premise checked, experiment specified, not run.
* **draft-PR skip** — F5. Policy decision.
