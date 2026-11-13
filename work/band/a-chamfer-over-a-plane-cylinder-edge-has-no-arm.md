---
id: a-chamfer-over-a-plane-cylinder-edge-has-no-arm
kind: issue
title: A chamfer over a plane–cylinder edge refuses ChamferArmUnsupported, so a bracket with a bore cannot be chamfered all round
status: closed
opened: 2026-11-12
priority: P1
cost: M
closed: 2026-11-13
pr: 6904
---


Found by the bracket step-3 lane (band-bracket3, phase 1, 2026-11-12) planning `a-boss-on-a-plate-meets-it-at-a-mixed-convexity-corner-no-run-out-rounds`.

The bracket witness (`crates/sweep/tests/pins_of_todays_blend_refusals.rs:59`) is a 6 × 4 × 0.5 plate with a boss and a two-arc bore. Once its mixed corners build, the **fillet** all round builds. The **chamfer** all round still refuses at the bore: the chamfer's arm table holds one arm, plane–plane (`crates/sweep/src/blend/mod.rs:1754`, `BlendError::ChamferArmUnsupported`; recourse `CHAMFER_ARM_RECOURSE` at `mod.rs:1460`), and the bore's rim edges are plane–cylinder. So step 3 pins the chamfer on a bracket without the bore.

The geometry is exact. A chamfer of a circular plane–cylinder edge at equal setbacks is a cone frustum coaxial with the cylinder, meeting the plane in a circle and the cylinder in a circle. So this is a missing arm, not a missing surface kind.

P1: "chamfer all round" on an ordinary part is the chamfer twin of the P0 the bracket row fixes.

## Closed

PR 6904. The chamfer's arm table has a second row, `BlendArm::CylinderPlaneCone` (`arms::chamfer_frustum`): a whole rim where a cylinder meets a plane square to its axis is replaced by the cone frustum through the two trim circles, carved by the fillet's closed-rim walks (ladder, seam-split and one-edge annulus), its sense derived by `arms::frustum_sense`. Pinned at `ΔV = ∓π·s²·(R ± s/3)` on both material sides at `f64` and `Interval` (`band_chamfer_frustum`), names through editor-core (`edit_bore_chamfer`). The bracket chamfered all round now refuses only at its mixed corner, the bracket row's ground. An arc inside a chain of lines and arcs keeps a typed refusal, filed as `a-chamfer-over-an-arc-between-lines-has-no-strip-to-frustum-junction`.
