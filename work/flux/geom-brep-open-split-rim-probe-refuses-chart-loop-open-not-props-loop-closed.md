---
id: geom-brep-open-split-rim-probe-refuses-chart-loop-open-not-props-loop-closed
kind: issue
title: geom-brep's rim_dim_review_probes open-split row is red: a loop open by 50eps now refuses ChartLoopOpen, not NotIsoRectangle props_loop_closed
status: closed
pr: 5369
closed: 2026-10-19
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

## Resolution (FLUX, `flux/main-reds-golden-and-split-rim`)

`ChartLoopOpen` is the intended verdict. A loop open by 50ε is a
genuine open junction, 50ε past the periodic charts' `3ε` junction
slack, and since #4839 that junction decide is where an open loop on a
cylinder refuses. The row's expected error is re-stated and its subject
kept: the open loop refuses on its closure, it does not measure.

The probe pre-pass on main plus this fix (every `plain` suite of the
census roster at 1e-9, plus all of geom-brep with `--features probe`
at 1e-6, 1e-9 and 1e-12) found one more stale FLUX expectation:
`rim_dim_scale_twins::the_cone_flux_lane_decides_lengths_at_both_scales`.
PR #5333 re-pointed its name list from `props_cone_loop_winding` to
`props_chart_loop_turns` and `props_chart_loop_sense`, but none of its
faces winds the axis, so `props_chart_loop_sense` never decided. A band
between two whole rims now joins its faces. Everything else is green.
