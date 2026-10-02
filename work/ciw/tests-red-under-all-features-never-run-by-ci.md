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
