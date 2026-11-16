---
id: the-ray-wall-twins-read-the-angle-predicates-off-scale
kind: issue
title: sweep's ray-wall margin twins stopped on azimuth_near_cut_side and period_near_mark, not the pinned curve_foot_branch: red on main behind the probe feature
status: closed
opened: 2026-11-06
priority: P2
cost: M
closed: 2026-11-09
pr: 6597
branch: onecopy/twins-dimension
---


Found by `onecopy/curve-pair-tidy` (PR 6541), running `sweep` with `--all-features`. Main 230be11d is red alike.

## What

`crates/sweep/tests/ray_wall_margin_twins.rs` `the_wall_arms_margins_scale_linearly_with_the_model` is pinned to stop on `["curve_foot_branch: 62 pairs off, e.g. 9.990e2"]` at ε = 1e-9 (`:97`). It now stops at `:163`, with `predicates whose margins do not scale with the model: ["azimuth_near_cut_side: 56 pairs off, e.g. 6.811e2", "period_near_mark: 208 pairs off, e.g. 9.990e-1"]`.

Both rows are the angle reads' (`geom::Azimuth::near`, `geom` period marks), which came in with ONECOPY's angle-reads work (PR 6482). The file is `#![cfg(feature = "probe")]`, and the nightly `dev-probe` row only lists `--test all` (`--list`, `.github/workflows/nightly.yml`, "compile and list every probe-gated test target"), so no CI run executes it. See `work/ciw/probe-gated-lib-unit-tests-never-run-in-ci.md`.

## Owed

Decide whether the two angle predicates' margins should scale with the model. A margin read in radians does not, and the twins' scale-linearity claim covers metres only. Then either fix the read, or re-pin the twins' stop with the reason.

## Merged

`ray-wall-twins-read-f31-angle-renames-as-margins-that-must-scale` (P3 E, filed by `onecopy/main-reds-k-and-m10`) was this finding filed twice. It is folded in here, and its file is gone. It measured the two predicates:

- `period_near_mark` (`geom_core::Period::near`): 208 pairs off, with ratio 1. Its margin `reference − t` is an angle difference, so it does not scale at all.
- `azimuth_near_cut_side` (`geom::Azimuth::near` through `stable_azimuth`): 56 pairs off, with a ratio of about a million. Its margin `c·y − s·x` reads the direction's components at whatever dimension the caller passes.

Both are taken under `k_stats::unlogged("F31", ..)` at `Band::least()`, a sign-only band, and the twins' verdict streams already matched. It offered two ways out:

1. Read only margins decided against a band with a length.
2. Lever the F31 margins to a length.

It recommended (1).


## Closed

PR 6597. The twins' scale claim covers margins classified against ε (D4 `[tolerance.margins]`). A sample decided at `Band::least()` is read for its sign only, and its record says so through `MarginSample::reads_sign_only` (`geom_core::k_stats`). The ray-wall twins and `topo`'s `rim_dim_boolean_twins` both read that method, and the pin now holds at 1e-6 only. The probe suite was already executed by the nightly (`RUN_FLOOR`); the ciw row is corrected.
