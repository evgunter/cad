---
id: tests-red-under-all-features-never-run-by-ci
kind: issue
title: Three tests fail on main when built with --all-features, and no CI row runs tests that way
status: open
opened: 2026-10-02
priority: P2
cost: E
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
