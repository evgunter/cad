---
id: the-cone-frusta-sizing-row-sits-on-the-one-second-bar
kind: issue
title: mesh's cone_frusta_mesh_within_delta took 1.050 s against the fast set's 1.0 s bar and turned a workflow-only PR's test (app) red
status: closed
opened: 2026-11-22
closed: 2026-11-22
priority: P4
cost: E
---


Filed by GREEN's `green/nightly-build` lane, from PR 7365's CI run
45040728595. That PR changes `nightly.yml`, a gate script and `work/`
only.

## What

`test (app)` went red on `scripts/check-fast-set-times.py`'s backstop:
`mesh::all sizing_theorem_rows::cone_frusta_mesh_within_delta took 1.050
s (existing test: 1.0 s)`. The branch's previous run (45034104026, head `c1f4a5f9d3`)
passed this check. It is the same class as
`the-snowman-pole-cut-row-sits-on-the-one-second-bar` and
`a-fast-set-germ-row-sits-on-the-one-second-bar-and-reds-a-docs-only-commit`:
a fast-set test whose hosted time sits at the bar, so runner noise
decides the colour.

## Fix

Move it to the slow set (`.config/nextest.toml`'s `ci` filter), or cut
its cost. Read its junit times over the last nights first.

## Closed (2026-11-22, green/fast-set-margin)

The row is in the slow set, one of four fast-set tests moved together
as an interim while PR 7397 decides the bar's policy. Over 40 PR runs
(36 readings) its gate p90 was 0.913 s and its median 0.665 s; its one
reading over the bar (1.050 s) was on a runner reading 1.41x the
sample's median. Uncontended (`-j1`, at the gate's build) it took
0.415-0.440 s, and 0.407 s on the nightly at opt 2
(`docs/perf-data/fast-set-times/`).
