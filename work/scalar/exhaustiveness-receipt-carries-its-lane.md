---
id: exhaustiveness-receipt-carries-its-lane
kind: unit
title: Exhaustiveness and ExhaustivenessInconclusive carry a lane tag with the chart lane's SupSpeed; meters by one method
status: closed
opened: 2026-09-15
branch: scalar/exhaust-lane
pr: 2667
closed: 2026-09-15
---


## What

`D283`'s ruling, route A (PR 2457), second unit, after
`rate-pair-in-geom-core`. `Exhaustiveness` and
`SsiError::ExhaustivenessInconclusive` (`geom-brep/src/ssi/exhaust.rs`)
gain a lane tag — `R3`, or `Chart { speed: SupSpeed }` — keep `floor`
and `cell_width` in the lane's own units (the units the
`cell.width() <= floor` decision was made in), and derive the meters
reading by one method through the rate type; `Display` names the lane.
`plane_nurbs_ssi`'s three divisions (seed floor, tube pad, accounting
floor) go through the same `SupSpeed`; the test helper
`assert_floor_is_the_meters_floor_over_the_chart_speed` and its pinned
`WALL_CHART_SPEED` retire. No dimensional newtype. TRIM's ground
(`ssi*`); announce. Full v6 dual.

## Closed (2026-09-15) — PR 2667

`ExhaustLane { R3, Chart { speed: SupSpeed<f64> } }` on `Exhaustiveness`
and on `SsiError::ExhaustivenessInconclusive(ExhaustivenessRefusal)`;
`floor`/`cell_width` in the lane's own units; metres by one method
(`ExhaustLane::meters`, with `speed()` the public reader); both
`Display`s name the lane and print the metres reading beside the lane
value through one `write_chart_length`, the refusal naming the
certified speed. The two chart doors (`seed_chart_plane`,
`account_chart_plane`) take `(speed, floor_meters)` and cross once;
`sweep` tallies, the doors attach the lane. `WALL_CHART_SPEED` and its
helper retired — the kernel's own rate reproduces the literal bit for
bit and the FLOOR-TIE rows assert `floor_meters()` within the five
roundings named. `Box3::speed_sup` is the one home of the
derivative-box sup; the limb-3 chart tube pads go through `to_param`.
Bit identity held (the receipt and refusal bits, every count).
Reviews: dual, both APPROVE WITH FIXES, one MAJOR bilateral (the filed
TRIM row's zero-speed mechanism was false by execution: a zero chart
speed certifies silently over the wrong cell — the row now says what
runs); twelve fix-pass items taken. Rows: TRIM's
`limb-3-chart-tube-speed-has-neither-guard-its-sibling-site-has`
(corrected), `ssi-tube-pad-folds-both-axes…` (re-worded).
