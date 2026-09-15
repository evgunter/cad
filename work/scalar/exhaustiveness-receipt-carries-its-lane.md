---
id: exhaustiveness-receipt-carries-its-lane
kind: unit
title: Exhaustiveness and ExhaustivenessInconclusive carry a lane tag with the chart lane's SupSpeed; meters by one method
status: open
opened: 2026-09-15
branch: scalar/exhaust-lane
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
