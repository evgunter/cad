---
id: the-demos-job-is-pr-cis-long-pole-on-its-release-build
kind: issue
title: PR CI's median (21.8 min) is the demos job's: its release build takes 20 min and it is the last job to finish in 44 of 61 runs over 20 min
status: open
opened: 2026-10-24
priority: P4
cost: M
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
