---
id: ssi-limb-three-refuses-a-bent-chart-path-line-at-eps-1e-12
kind: issue
title: ssi: limb 3 refuses the bent-chart-path flat wall's line at mu 0.1 and eps 1e-12, its pieces stopping short of a carrier end
status: open
opened: 2026-10-06
priority: P3
refs: [limb3-carrier-ends-read-at-f64-points]
---


(Filed by the transversality-lever lane, 2026-10-06; first seen by
lever B in the design fork, round 1.)

## What

`m5_pr7_ssi::a_flat_wall_whose_chart_path_bends_answers_as_the_plane_it_is`
(`crates/geom-brep/tests/m5_pr7_ssi.rs`, `bent_path_flat_wall`): the
plane `z = 0` cuts the flat wall `z = 1e-6·x`, `X` 1 cm, `κ` 0.8, in the
line `x = z = 0`. At ε 1e-12 and `μ` 0.1 the door refuses
`SsiError::TubeNotOneArc`: "at 19 rungs the uniqueness tube was a graph
but not proved one arc; at the narrowest, the pieces stop short of an
end of the carrier". `μ` 0 and 0.2 answer the line at the same ε, and
`μ` 0.1 answers it at 1e-9. The refusal is the same on main (chart
arm) and under every lever the fork measured, so it is not the lever.

The geometry is one straight branch between two side crossings, so the
refusal is wrong in the safe direction. The suspected cause, not
established, is the f64 read of the carrier's ends
(`limb3-carrier-ends-read-at-f64-points`): the end slice is read
within ε of a rounded end point, and at 1e-12 on a 1e-6 slope that
rounding is a sizeable part of ε.

The row stands down on this one configuration, naming this file.

## Repair shape

Find which end check fails (`one_arc::reaches_end` in
`crates/geom-brep/src/ssi/certify.rs`), and whether enclosing the end
cures it; then lift the row's stand-down.
