---
id: the-demos-job-is-pr-cis-long-pole-on-its-release-build
kind: issue
title: PR CI's median (21.8 min) is the demos job's: its release build takes 20 min and it is the last job to finish in 44 of 61 runs over 20 min
status: closed
opened: 2026-10-24
priority: P0
cost: M
parent: pr-ci-wall-clock-root-cause-measured
closed: 2026-10-30
---

## Found (INTENT nightly triage, 2026-10-24)

Nightly run 40066431545's `PR CI latency (alarm)` reported `n=101
median=21.8 min p90=32.0 min (alarm at median > 20)`. The alarm's
statistic was recomputed from `actions/workflows/ci.yml/runs`
(`event=pull_request`, success or failure, created 10-23 05:29Z to
10-24 05:29Z): n=105, median 21.8. Every run's jobs were then read
from `actions/runs/<id>/jobs`.

- **Queueing is not the cause.** Every job's median queue (created →
  started) is 0.0 min.
- **The run that ran `demos (tour + wild)` is the slow one.** The 49
  runs that ran it have a median wall of 29.6 min. The 56 that did not
  have a median of 8.7 min.
- **`demos` is the last job to finish** in 44 of the 61 runs over
  20 min. `test` is last in the other 17. Job medians: `demos` 28.4
  (p90 33.4), `test` 18.5 (p90 25.1), `python` 9.1, `lint` 5.4.
  On 10-16 (`pr-ci-wall-clock-root-cause-measured`) `test` was the
  long pole at 24.8 min and `demos` sat under it at 25.0. `test` has
  since come down. `demos` has not, so it is now the critical path.
- **Inside `demos`, the time is the release build.** Of 12 sampled
  jobs, the `demos tour suite` step's median is 26.8 min.
  `fmt + clippy` takes 1.0. One job read in full (run 40065265746,
  job 120787243893): `Finished release profile ... in 20m 32s`, then
  86 tests in 512.9 s. That 512.9 s is one row,
  `eps_regression tour_runs_green_at_eps_1e_12`. The next rows are
  `gallery::tests::each_gallery_document_denotes_its_scene_or_says_why_not`
  (155.7 s) and `eps_regression certified_cells_run_green_at_default_eps`
  (132.8 s).
- **The branches that drove the median** each ran `demos` on every
  push: `quad/measure-a-lanes` (6 runs, median 33.9),
  `intent/census-planar-face-pair` (6, 30.1),
  `intent/mirror-seam-walk` (5, 30.4), `iso/one-polynomial-weight-test`
  (5, 30.0), `intent/dprime3-assemble-product-slot` (5, 29.0),
  `intent/dprime2-mates-read-geometry` (5, 28.5),
  `quad/patch-doors-one-net` (7, 27.7), `quad/measure-b-door` (7, 26.5).

## Owed

Take the demos job off the critical path. Candidates, to be measured
before one is chosen:

- build the tour in a profile cheaper than full `--release` (for
  example `opt-level` 1 or 2, or fewer codegen units' worth of LTO),
  if its 1e-12 row stays inside the runner's time at that profile;
- move `tour_runs_green_at_eps_1e_12` (513 s, one row) to the nightly,
  the way the latency cut moved the kernel's extra-ε rows;
- check whether `demos` selects on diffs that cannot reach the tour.
  Every kernel diff runs it now, by design
  (`demos-row-skips-kernel-changes-to-doors-the-tour-calls`).

The 20-min alarm threshold stays as it is.

## Measured, 2026-10-28 (GREEN latency lane)

Over the 186 completed PR runs in the 24 h to 10-28 05:30Z, `demos` ran
in 54 (median 28.7 min, p90 29.8). It was the last job to finish in 44,
all of them over 20 min. `demos tour suite` has a median of 27.2.

**The release build, split.** Two full logs (jobs 122320999843 and
122193702486):

| job | kernel and its deps | `demo-tour`'s own units | whole build | tests |
|---|---|---|---|---|
| 122320999843 | 8.6 min | 13.7 min | 22m15s | 310 s |
| 122193702486 | 9.3 min | 14.0 min | 23m16s | 331 s |

The kernel part runs from the build's start to `Compiling demo-tour`;
the tour part runs from there to `Finished`. The tour's own units are
the bin, the bin's test harness (32k lines of `src/`, compiled twice) and
ten integration-test binaries.

**Declined: moving `tour_runs_green_at_eps_1e_12` to the nightly.**
- In three jobs it ran 310, 331 and 310 s. The next-longest rows ran
  245, 170 and 239 s, so the suite's wall would fall by 1–2.7 min, on
  the ~29% of runs that run `demos`.
- Replayed over the window, `demos` −5 min moves the alarm's median
  from 23.1 to 22.9.
- The row is the tour's one row with unshared PR catches
  (`work/ciw/demote-unearned-tests.md`: "`_at_eps_1e_12` stays, because
  every PR red of the other two was also its red"). Ev's demotion bar
  keeps such a row per-PR, and this payoff does not argue for an
  exception.

**What remains.** The `test` split (green/latency-test-shards) takes the
median off `test`. `demos` then holds the p90 at about 29 when it runs,
which is past the 25-min deep-kernel bar. The lever is the tour's own
14 min of build: two copies of the 32k-line bin and ten test binaries,
each instantiating the kernel's generics in release. The candidates to
measure, with `cargo build --timings` on a hosted run, are:
- fold the integration-test binaries into one `all` binary, as the
  kernel crates did (`autotests = false`);
- move the bin's unit tests out of `src/`, so the bin is compiled once.

## The test binaries folded, 2026-10-29 (GREEN latency lane, #5989)

`demos/tour`'s ten `tests/*.rs` binaries are now one, `all`
(`autotests = false`, `tests/all.rs` with `#[path]` mounts as the kernel crates have, helpers in `tests/common/mod.rs`).
The test set is the same, listed from the built binaries before and after:
97 tests before; afterwards the same 97 as `all::<suite>::<test>` plus the
kernel crates' aggregation row `every_suite_file_is_aggregated` (test-utils is now a dev-dependency). The `ci` profile's filter and
the nightly's probe row name the new binary.

**Local, `cargo test --release --no-run --timings`, 4 cores, the tour's
own units only (the kernel already built):**

| layout | tour phase | tour unit-s | bin | bin's test harness | integration tests |
|---|---|---|---|---|---|
| ten binaries (main) | 743 s | 2,851 | 743 | 736 | 1,371 (10 units) |
| one `all` | 538 s | 1,430 | 531 | 538 | 361 |
| one `all`, no harness (measurement only) | 346 s | 585 | 346 | — | 239 |

**Hosted.** Runner speed varies by about 1.5× between jobs, so each
row gives the kernel part of the same build as its yardstick:

| job | layout | kernel part | tour part | build | tests | job |
|---|---|---|---|---|---|---|
| 123247133788 (#5984) | ten binaries | 6.5 min | 9.9 min | 16m21s | 178 s | 20.7 min |
| 123243230894 (another PR) | ten binaries | 9.8 min | 14.9 min | 24m45s | 315 s | 31.7 min |
| 123259033963 (#5989) | one `all` | 9.6 min | 10.2 min | 19m45s | 284 s | 26.1 min |

Against the slow-runner job with the same kernel time, the tour part
fell from 14.9 to 10.2 min and the job from 31.7 to 26.1. The ratio of
tour part to kernel part is 1.52 in both ten-binary jobs and 1.06 after.

**Still the p90 tail, so this stays open.** The tour part is now the bin
and its unit-test harness compiled side by side, each about 530 unit-s
locally. Without the harness the tour part measured 346 s, another
−190 s. That lever is `demo-tour-unit-tests-compile-the-bin-twice`.
Splitting a lib out of the bin does not reach it on its own: the lib's
`#[cfg(test)]` harness compiles the same 32k lines a second time.

## The bin compiled once, 2026-10-29 (GREEN, PR #6039)

`demo-tour-unit-tests-compile-the-bin-twice` is closed: the tour is a
lib with a thin bin and no harness. On one hosted runner, with the
kernel warm, the tour phase went from 648 to 453 s (−3.3 min). The
units are now a serial chain (lib 268 s, then `all` 184 s), so `all`
is what is left to cut. The moved unit tests re-instantiate the
scenes' generics for `f64` inside `all`, because release builds do not
share generics across crates.

**Still open.** A `demos` job at its pre-change median of 28.4 min
less 3.3 min is about 25 min, at the `test` job's p90 (25.1). Whether
`demos` is still the p90 tail has to be read from the PR runs after
this lands. The kernel half of the build (8–10 min uncached) is
untouched.

## Closed (2026-10-30)

Measured over ci.yml `pull_request` runs from 2026-10-29T20:09Z to 2026-10-30T11:09Z, after #6019 and #6039 (n=116, success or failure, first attempts):
- run wall: median 14.3 min, p90 24.1;
- by tier: closure median 14.6 / p90 19.4 (n=68); all median 24.1 / p90 25.3 (n=15); docs 6.4 (n=33);
- test shards at the closure tier, paired (n=52): kernel 13.4 against app 12.2; kernel longer in 39;
- `demos` when it runs: median 22.8, p90 23.7 (n=22).

The nightly alarm read a median of 14.7 min on nightly 77. That is inside the 15-min bar at the median. The all tier, which builds the whole workspace, sits inside the 25-min bar `memories/orchestration-model.md` sets for a diff that seeds the deep kernel crates.

`demos` still finishes last in 20 of the 22 runs where it runs, but at
22.8 min those are all-tier runs inside their 25-min bar. Closing on
that. A further cut has to come from the tour's `all` binary
recompiling the scenes' generic code for `f64` (the lane's note on
#6039), and should be filed when a run needs it.
