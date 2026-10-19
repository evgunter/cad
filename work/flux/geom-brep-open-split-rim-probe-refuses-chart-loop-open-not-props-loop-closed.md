---
id: geom-brep-open-split-rim-probe-refuses-chart-loop-open-not-props-loop-closed
kind: issue
title: geom-brep's rim_dim_review_probes open-split row is red: a loop open by 50eps now refuses ChartLoopOpen, not NotIsoRectangle props_loop_closed
status: open
opened: 2026-10-15
priority: P1
cost: E
---


## What

`geom-brep::rim_dim_review_probes::small_body_rims_50eps_apart_refuse`
(`crates/geom-brep/tests/rim_dim_review_probes.rs`, a
`feature = "probe"` suite, ~line 116) is red at ε 1e-9 on main
(`736a494f87`, 2026-10-15):

```text
panicked at crates/geom-brep/tests/rim_dim_review_probes.rs:120:5:
a loop open by 50eps must refuse on its closure, not measure: Err(ChartLoopOpen)
```

The row asserts that `curved_face` refuses the open split-rim loop with
`PropsError::NotIsoRectangle { what: "props_loop_closed" }`. It now
refuses `ChartLoopOpen`. That is still a typed refusal of the loop's
closure, so this reads as a stale expectation rather than a measured
open loop. The likely mover is FLUX's junction-closure decide
(`1dd60ee728`, "junction-closure decide and integer windings replace the
half-turn decides"; reached main with #4839, 2026-10-12), which refuses
a genuine defect as `ChartLoopOpen` or `LoopsWindChart` before the Green
form's own closure check. Not bisected.

The suite is rostered `plain:geom-brep:rim_dim_review_probes` in
`scripts/gates/probe-suite-census.sh`, so `scripts/k_probe_sweep.sh`'s
`run_plain` pre-pass stops on it and nightly `k-lint (dev-probe)` lints
nothing while it is red. The PR gate never runs it.

Repro: `CAD_TOLERANCE_EPS=1e-9 cargo test -p geom-brep --features probe --test all -- rim_dim_review_probes::`

## Owed

FLUX decides which refusal an open loop on this surface should carry.
If `ChartLoopOpen` is the intended verdict, re-state the row's expected
error (its subject, that the open loop refuses on its closure instead of
measuring, is kept). If `props_loop_closed` should still be the one that
fires, the junction decide is pre-empting it and that is a FLUX fix.

Found by CIW's `ciw/silent-fixed-predicates-probe` lane running the
whole `run_plain` selection. The test file is TCOST/TINT ground; the
behaviour that moved it is FLUX's.
