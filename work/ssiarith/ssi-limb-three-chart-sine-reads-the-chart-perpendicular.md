---
id: ssi-limb-three-chart-sine-reads-the-chart-perpendicular
kind: issue
title: ssi: limb 3's chart sine is read along the chart-perpendicular direction, not the surface-perpendicular one
status: open
opened: 2026-10-06
priority: P3
---


(Filed by the transversality-lever lane, 2026-10-06; found by lever A
in the fork on
`work/ssi/ssi-transversality-at-a-point-is-spelled-three-ways.md`.)

## What

Limb 3's chart probe reads its sine-like margin as `∇φ · e⊥` over the
chart's stretch along `e⊥`, where `e⊥` is the chart direction
perpendicular to the pcurve's tangent (`chart_transverse_margin`,
`crates/geom-brep/src/ssi/enclose.rs` ~448). That direction is
perpendicular to the locus on the surface only where the chart is
orthogonal there. On a skewed chart the margin is a lower bound of
the true sine (so it can only refuse more), but the skew does not
shrink with the rung, so a skewed chart can refuse `TubeStraddles`
however narrow the tube.

Measured: on the near-degenerate hyperbola, reading the margin along
the surface-perpendicular direction moved nothing (lever A, round 4,
`LEVER_METRIC`); no row on main is known to move.

## Repair shape

Take the transverse direction perpendicular on the surface (from the
first fundamental form) rather than in the chart.
