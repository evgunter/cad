---
id: ssi-boundary-strip-sine-divides-by-the-whole-walls-speed
kind: issue
title: ssi: the boundary strip's sine divides by the whole wall's sup speed, not the strip's own
status: open
opened: 2026-10-06
priority: P3
---


(Filed by the transversality-lever lane, 2026-10-06; found by both
designers in the fork on
`work/ssi/ssi-transversality-at-a-point-is-spelled-three-ways.md`.)

## What

The boundary pass's strip classification (`side_region`,
`crates/geom-brep/src/ssi/boundary.rs` ~465) reads the sine of the
angle across a side as the strip's least slope over the wall's sup
chart speed along that axis: `div_down(inf, speed.get())` with `speed`
from `self.speeds`, minted once over the whole wall. Where the chart's
speed varies, the strip's own speed is smaller than the wall's sup, so
the sine reads smaller than the strip's. That is conservative, but it
reads the chart's parameterisation, and the slack does not shrink as
the strip narrows: a chart that runs fast elsewhere can refuse
`SsiError::BoundaryTangent` at a side whatever the rung.

## Repair shape

Divide by the strip's own sup speed (the per-piece `speed_sup` the
side cover already reads, `boundary.rs` ~1167).
