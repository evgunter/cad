---
id: tests-red-under-all-features-never-run-by-ci
kind: issue
title: Three tests fail on main when built with --all-features, and no CI row runs tests that way
status: closed
opened: 2026-10-02
priority: P2
cost: E
closed: 2026-10-28
---


## What

The TANG lane on PR 3795 ran the workspace's tests with
`--all-features` and found three that fail. They fail identically on
`origin/main` at `b4c1de87` (reported by the lane; not re-run by the
TANG orchestrator):

- sweep `r1_ring_clearance_decisions_per_carve`
  (`crates/sweep/tests/review_ring_clearance_r1_probes.rs`)
- topo `boolean_margin_streams_scale_linearly_with_the_model`
  (`crates/topo/tests/rim_dim_boolean_twins.rs`)
- pncad-py `prose_census` (a `viewer/src/idpass.rs` entry)

`.github/workflows/ci.yml:~183-196` runs clippy with `--all-features`
but runs no tests that way. `nightly.yml:~141-144` covers only the
viewer's docs and clippy. Nothing therefore catches a test that a
feature-gated path breaks.

## Owed

- Diagnose each of the three. They may belong to the owners of the test
  files (territory: tcost, tint) or of the feature-gated code.
- Decide whether a `--all-features` test leg belongs at the PR gate or
  nightly.

## The sweep and topo reds: measured and fixed (REACH, 2026-10-02)

Both are `#![cfg(feature = "probe")]` suites, and a CI leg DOES run
them: the nightly's `k-lint (dev-probe)` row, through
`scripts/k_probe_sweep.sh`'s default-selection loop at ε 1e-9. That row
has been red on them since #3715 (nightly run 36999416593, job
110813608440: `r1_ring_clearance_decisions_per_carve`, rim (1,0) 2 vs 0).
Both are fixed on `reach/dev-probe-red`: the sweep row's counts were
stale against #3715's support-boundary walk, and the topo row caught a
real defect, a unit-at-rest normal's dimensionless norm decided against
the length band (`bool_germ_plane_normal`, #3768). The pncad-py
`prose_census` entry is untouched and still owed here.

What let them land and stay, which this row's premise did not name:

- **The PR gate runs no probe suite.** `ci.yml` compiles the probe
  targets (clippy `--all-features`) but never executes them, so a diff
  that reds one merges green; only the nightly sees it, after the fact.
- **The sweep's plain loop stops at the first red suite.** `run_plain`
  runs each rostered suite under `set -e`, in roster order, so the
  sweep red aborted the loop before any `topo` suite ran: the topo
  twin's red, and before #3795 the `rim_dim_review_probes` one, never
  appeared in a nightly log. A red early in the roster masks every
  red after it.
- **The nightly was red on other rows at the same time** (the full
  suite, rustdoc, the tessellation-budget lint), so one more red row
  read as noise rather than as a new merge's.

Owed: run the probe suites of the crates a diff touches at the PR gate
(or say why not), and make the plain loop run every suite and fail at
the end with the list of red ones.

## A fourth instance (CLEAVE, 2026-10-03)

sweep `a_carved_balls_meridian_fragments_record_no_clearance_charge`
(`crates/sweep/tests/tilted_sphere_pair_k_rows.rs`) went red on main
when #3805 renamed the K predicate it filters on
(`bool_circle_curved_clearance` → `bool_conic_curved_clearance`): a
cross-PR semantic conflict that only a probe-suite execution can see,
and the PR gate runs none. Fixed in
`work/cleave/a-sweep-row-fails-under-all-features-on-main.md`'s PR; the
gap it went through is this row's first bullet above.

## A fifth instance (PCERT, 2026-10-12)

Seven `geom-brep` probe rows (`m8_f67_r1_probes` ×2,
`span_meter_dim_twins` ×5) went red on main when #4791 made the
lane-free `EdgeCurve::certify` refuse `NurbsLaneNotSupplied` for an
analytic rung-3 edge; the rows certified that class lane-free. Found by
PR 4846's `--all-features` run, fixed by PR 4876
(`work/pcert/probe-rows-refuse-nurbs-lane-not-supplied-at-the-lane-free-door.md`).
The gap is this row's first bullet again.

## Closed (2026-10-28)

**The three named rows are green on main under `--all-features`**
(`4beeeb05`), each run on its own:

- `cargo test -p sweep --all-features --test all -- r1_ring_clearance_decisions_per_carve`
- `cargo test -p topo --all-features --test all -- boolean_margin_streams_scale_linearly_with_the_model`
- `cargo test -p pncad-py --all-features --lib -- prose_census` (18 of 18;
  the crate's test targets link with `extension-module` on)

REACH fixed the first two on 2026-10-02. The pncad-py entry was fixed
somewhere along the way, so nothing was owed there.

**The row is added.** `nightly.yml` gains `suite under --all-features
(default eps)`. It runs `cargo nextest run --profile ci --workspace
--exclude viewer` with every workspace feature on, at default eps. The
slow set is left to the full-suite rows. `viewer` is excluded because its
`app` feature needs the GPU stack the viewer job installs, and that job
runs `-p viewer --features app` already.

**What a whole-workspace `--all-features` run found first** (hosted run
`40720990736`): 14 009 tests, 24 red, in three causes. Per the GREEN
policy that a lane does not fix kernel causes, each one is pinned or held
out with the row that owns it:

- **21 sweep rows** fail with `mfkrh postcondition … OrphanGeometry
  { Surface }`, under topo's `per-op-postcondition` scalpel. That
  item is `work/topo/mfkrh-leaves-an-orphan-surface-under-the-per-op-scalpel.md`
  (open, P2; today's list was added to it). The nightly row holds the
  scalpel out, along with every feature that forwards to it
  (`pncad/per-op-postcondition`), through `HELD_OUT_FEATURES`. A
  transitive check derives this, and it fails loudly if a held-out
  feature stops existing. Bringing the scalpel back into the row is
  `work/ciw/no-ci-row-runs-topo-under-the-per-op-postcondition-scalpel.md`,
  which now says so.
- **2 topo rows read the default build** under the scalpel: a
  tier-1-derivation count, and a `should_panic` message. They are now
  `#[cfg_attr(feature = "per-op-postcondition", ignore = "pinned red: …")]`,
  citing the new `work/topo/two-topo-rows-assume-the-default-build-under-the-per-op-scalpel.md`.
  Locally, under `-p topo --lib --features per-op-postcondition`, both
  fail before the pin and are ignored, with that reason, after it.
- **1 probe row**, `ray_wall_margin_twins::the_wall_arms_margins_scale_linearly_with_the_model`,
  is already red on main in `k-lint (dev-probe)` and owned by
  `work/green/main-reds-editor-core-slow-set-and-the-k-probe-sweep.md`.
  Another GREEN lane holds that row, so it is held out of this job by a
  nextest filter (`HELD_OUT_TESTS`) rather than edited.

**Cost, measured** with a temporary push-triggered copy of the job
(deleted before review). Run `40751709384` took **27 min build + 87 s
suite**, about 29 min in all: 14 008 run, 14 008 passed, 1 840 skipped.
The cold-cache build took 31 to 33 min, so the job keeps no Rust cache:
a warm build saves about 5 minutes, and a whole `--all-features` target
would compete with the PR gate's caches for the repo's 10 GB Actions
cache quota. The job runs in parallel with `full suite` (about 123 min),
so it does not lengthen the nightly. It adds about 35 runner-minutes a
night, and it adds nothing to the per-PR gate. One of the three measuring runs hung in `build` for
two hours. That is filed as
`work/green/nightly-all-features-build-hung-once-with-a-test-binary-running.md`,
and the step timeouts (60 min and 30 min) bound a repeat.

**Not done:** an `--all-features` leg at the PR gate. The probe-suite
part of the "owed" list above stays open on its own terms; this row's
ask was the nightly.
