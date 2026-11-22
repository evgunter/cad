---
id: the-cone-frusta-sizing-row-sits-on-the-one-second-bar
kind: issue
title: mesh's cone_frusta_mesh_within_delta took 1.050 s against the fast set's 1.0 s bar and turned a workflow-only PR's test (app) red
status: open
opened: 2026-11-22
priority: P4
cost: E
---


Filed by GREEN's `green/nightly-build` lane, from PR 7365's CI run
45040728595. That PR changes `nightly.yml`, a gate script and `work/`
only.

## What

`test (app)` went red on `scripts/check-fast-set-times.py`'s backstop:
`mesh::all sizing_theorem_rows::cone_frusta_mesh_within_delta took 1.050
s (existing test: 1.0 s)`. The same head's previous run (45034104026)
passed this check. It is the same class as
`the-snowman-pole-cut-row-sits-on-the-one-second-bar` and
`a-fast-set-germ-row-sits-on-the-one-second-bar-and-reds-a-docs-only-commit`:
a fast-set test whose hosted time sits at the bar, so runner noise
decides the colour.

## Fix

Move it to the slow set (`.config/nextest.toml`'s `ci` filter), or cut
its cost. Read its junit times over the last nights first.
