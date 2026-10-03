---
id: lily-walls-curved-clearance-crowds-the-band-under-k-lint
kind: issue
title: k-lint (dev-probe) flags 40 bool_circle_curved_clearance margins on the lily_walls demo, unseen while the probe sweep was red
status: open
opened: 2026-10-02
---


## What

Nightly's `k-lint (dev-probe)` row lints the CSVs `k_probe_sweep.sh`
writes. The sweep has been red since #3715 (its plain loop reds on a
probe suite before the dumps), so the lint step has not run on main
since then. On `reach/dev-probe-red`, which greens the sweep, the lint
reads (local run, every ε row, 2026-10-02):

```
k-lint: GATE FAILED — the margin distribution changed: 75 margin(s) crowd a decision
     40 bool_circle_curved_clearance   (all demo/lily_walls; 12 / 14 / 14 at ε 1e-6 / 1e-9 / 1e-12)
     27 chart_bound_outer_span         (filed: work/chart/chart-bound-outer-span-decides-a-poisoned-margin.md)
      8 bool_circle_torus_root_slack   (filed: work/germ/circle-torus-root-slack-crowds-the-zero-band-at-1e-12.md)
```

The 40 `bool_circle_curved_clearance` margins (decided at
`crates/topo/src/boolean/reduce.rs`, the circle × curved-face
clearance) draw 46 FLAG lines, since a margin can break two rules at
once: 8 in the ambiguity band (indeterminate), 26 definite below the
baseline floor (4e-5), 6 definite within 10² of the escalation
threshold, and 6 zero-classified within 10² of the coincidence
threshold. At ε 1e-6 the lily wall's margins run from
6.5e-8 to 3.0e-6. That branch changes no margin under this name, and
its geometry is bit-identical to main's: `UnitVec3::levered` normalizes
exactly as `UnitVec3::new` did. So the flags are main's.

## Owed

Per the K-REPORT runbook: measure which lily-wall pairs these are, and
decide whether the baseline wants re-deriving or the clearance is
deciding a margin it cannot carry. Do not change geometry to get under
the threshold.
