---
id: an-extra-eps-step-that-selects-no-test-reads-green
kind: issue
title: A shard's 1e-6 and 1e-12 step that selects no test at all reads green, so a PR looks gated at rows where nothing ran
status: open
opened: 2026-11-14
priority: P3
cost: E
---


Filed by GREEN's default-eps lane (`green/default-eps`, PR 6931,
2026-11-14), from its STYLE review (C1), re-read from the step logs.

## Measured (run 43555174396, PR 6931 at `6ae8d4c99`)

`.github/workflows/ci.yml`'s step "test (eps 1e-6, 1e-12 — the
eps-sensitive crates and test files this diff touches)" ran in both test
shards with one expression,

    package(topo) | (binary_id(sweep::all) & test(/^a_plane_near_a_needle_cones_apex::/)) | (binary_id(sweep::all) & test(/^an_aimed_ray_from_far_reads_a_split_side_right::/)) | (binary_id(viewer::all) & test(/^door_at_rest::/))

- `test (app)` (`-p editor-core -p mesh -p pncad -p pncad-py -p verbs -p viewer`), at each of 1e-6 and 1e-12:
  `Starting 0 tests across 1 binary (1024 tests and 10 binaries skipped, including 2 tests via profile.ci.default-filter)`,
  `Summary [ 0.001s] 0 tests run: 0 passed, 1024 skipped`,
  `warning: no tests to run`. The step is green: it passes
  `--no-tests=warn`.
- `test (kernel)`: 3779 tests at each row, all of `topo` and, from the
  two sweep files, only the one fast-set test each has.

The diff's whole content in `viewer` is `door_at_rest.rs`, whose two
tests are both in the slow set, which `--profile ci` leaves out. So the
filter asked for a file at two ε rows, the profile removed everything
the file holds, and the step reported success. A reader of the PR's
checks sees "test (eps 1e-6, 1e-12 …)" green on the shard that owns
the file.

The same PR shows the neighbouring gaps, each with its own row:
the slow tests it edits ran once, at the default ε only
(`the-slow-tests-a-diff-adds-run-at-the-default-eps-only`); they ran at
all only once its comments spelled their names
(`an-edit-inside-a-slow-test-that-does-not-respell-its-name-runs-it-nowhere-on-the-pr`);
and the `viewer app feature` job ran 1551 tests with no `door_at_rest`
row among them, at the default ε
(`the-viewer-app-feature-rows-gate-one-eps-of-three`), though that job
is where the nightly showed the pair red.

## Wanted

A selected-but-empty ε step does not read as a pass. Either the step
says so where a reader looks (a notice in the step summary naming the
files whose rows the profile left out, as the change filter already
prints for the slow-set step), or the step is skipped when the
intersection of the expression, the shard's packages and the `ci`
profile is empty, so the check list shows it did not run. `--no-tests=warn`
stays for the shard the expression does not concern.
