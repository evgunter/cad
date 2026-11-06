---
id: the-ray-wall-twins-read-the-angle-predicates-off-scale
kind: issue
title: sweep's ray-wall margin twins stop on azimuth_near_cut_side and period_near_mark, not the pinned curve_foot_branch: red on main, unseen behind the probe feature
status: dispatched
opened: 2026-11-06
priority: P2
cost: M
---


Found by `onecopy/curve-pair-tidy` (PR 6541), running `sweep` with `--all-features`. Main 230be11d is red alike.

## What

`crates/sweep/tests/ray_wall_margin_twins.rs` `the_wall_arms_margins_scale_linearly_with_the_model` is pinned to stop on `["curve_foot_branch: 62 pairs off, e.g. 9.990e2"]` at ε = 1e-9 (`:97`). It now stops at `:163`, with `predicates whose margins do not scale with the model: ["azimuth_near_cut_side: 56 pairs off, e.g. 6.811e2", "period_near_mark: 208 pairs off, e.g. 9.990e-1"]`.

Both rows are the angle reads' (`geom::Azimuth::near`, `geom` period marks), which came in with ONECOPY's angle-reads work (PR 6482). The file is `#![cfg(feature = "probe")]`, and the nightly `dev-probe` row only lists `--test all` (`--list`, `.github/workflows/nightly.yml`, "compile and list every probe-gated test target"), so no CI run executes it. See `work/ciw/probe-gated-lib-unit-tests-never-run-in-ci.md`.

## Owed

Decide whether the two angle predicates' margins should scale with the model. A margin read in radians does not, and the twins' scale-linearity claim covers metres only. Then either fix the read, or re-pin the twins' stop with the reason.
