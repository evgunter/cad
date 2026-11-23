---
id: a-fitted-chords-projected-pcurve-certificate-is-taken-at-three-doors-per-operation
kind: issue
title: A ball × cylinder boolean takes its fitted chords' projected pcurve certificates at three doors, about 40 % of the operation
status: closed
opened: 2026-11-20
priority: P2
cost: M
closed: 2026-11-20
---


Found by SECT's `sect/ball-cylinder-cost` lane (2026-11-20), profiling
`a-ball-and-cylinder-boolean-costs-seconds-per-op-in-an-optimized-build`.

## Measured

The unit ball ∪ the rod of
`the_door_reads_seam_regions_as_tier_3_does.rs`, at the default ε,
opt-level 2, debug assertions off, after that lane's tube lever: 2.43 s.
150 gdb samples of the main thread over 30 repeats of the operation, of
which the projected pcurve checks (`pcurve_cache::projected`'s
`run_projected_checks`: check 4's `sweep_box` → `placed_hull` →
`frame_box_on`, the `aside` detached composite, and `param_rate_gate`'s
`speed_lower_bound`) take about 40 %, at three doors:

| door | samples |
|---|---|
| the site mint (`euler::plan_site_rows` → `site::site_rows` → `decision::certify_walked` → `PcurveCache::certify`) | 25 |
| the pcurve mint after the build (`pcurves::mint_pcurves` → `mint_rows` → `derive_face` → `stated::certify_stated_image` → `certify_projected`) | 18 |
| the result gate (`validate::tier3_local_checks_marked` → `tier3::pcurve_pass` → `PcurveCache::recertify`) | 16 |

Not measured: whether the three doors read the same pcurve on
bit-identical input (the samples show which doors run, not which row
each reads). If they do, this is the five-times row's shape
(`an-operation-retakes-each-carried-chords-rung-3-certificate-five-times.md`)
on the pcurve rows, and its fix is the same design question: a verdict
carried from the door that took it, or a memo keyed on the
certificate's whole input.

## What closing it takes

Count each door's certificates and their distinct inputs on that
operation first; then either share them (the design question above,
put to the designers) or make check 4's sweep cheaper on a fitted
carrier.

## Closed (2026-11-20)

By `a-certificate-is-read-by-its-support`. Every site-mint call on a
projected net's row refused (`FittedLaneUnsupported`, the site holding
no fitted door) after checks 1–3; the lane-free door now certifies no
row only the fitted door certifies (`Pcurve::needs_fitted_door`), so
check 4 is read one way, with the door held. The gate and `carry_rows`
read a row's certificate where its support compares the same. Ball ∪
rod: 32 projected checks → 8, one per kept row, at `mint_pcurves`.
