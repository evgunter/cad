---
id: param-rate-re-spells-a-carriers-speed-floor-per-kind
kind: issue
title: geom-brep: pcurve_cache's param_rate re-spells a carrier's speed floor per kind beside carrier_speed, a line's as 1
status: open
opened: 2026-11-06
priority: P3
cost: E
---


Found by the sweep of the `onecopy/boolean-standing` lane, which
closed `work/onecopy/carrier-speed-is-re-spelled-with-a-default-arm-in-import.md`.

## What

`geom_brep::pcurve_cache`'s `param_rate` meters a carrier's
parameter into metres, matched per kind:

- `Curve3::Line(_) => InfSpeed::new(T::one())`, where a line's speed is
  `|dir|` (`geom::carrier_speed::speed_range`, exact on a line). `dir` is
  unit only by convention (`geom::Line`'s docs: "the readings divide by
  `|dir|²`, so they hold for any nonzero `dir`"). So a carrier with a
  non-unit `dir` is metered at 1 m per parameter unit, not at its speed.
- `Curve3::Nurbs` reads `speed_lower_bound`, the conics and the spiric
  read `carrier_speed::speed_floor`.

That is a second per-kind table of a carrier's speed floor beside
`carrier_speed`, which DESIGN's crate table makes the home of "a
carrier's rate bounds".

## What closing it takes

Read the floor from `carrier_speed::speed_range(carrier).0` on every
kind. For a NURBS net, check first that it equals `speed_lower_bound`
(`carrier_speed`'s docs say it is the net's bound). The gate
`param_rate_gate` keeps refusing a poisoned net. Then measure whether
any row stores a non-unit line `dir` that the change re-meters.
