---
id: the-area-pass-never-refines-beside-a-converged-volume
kind: issue
title: The rational patch lane's area pass is fixed-resolution and never refines, so a face reported converged on its volume carries a percent-wide area
status: open
opened: 2026-10-21
priority: P2
cost: M
---


Found off-question by a designer weighing `quad2-rational-max-rounds-dial-decision`
(2026-10-21, on `quad/check7-sign-walk`); not yet reproduced by the filer.

The reporting target meters the FLUX only. In `crates/geom-brep/src/props/quad/`,
the area pass of the patch lanes runs at a fixed resolution
(`AREA_CELLS_PER_SPAN`) and never refines with the rounds. Measured on the
quarter torus: the area bracket is 0.47 m² wide (about ±2.4% of π² m²) on every
round, and it is reported beside a volume the lane calls converged. `area_pad`
is honest about the width; the "converged" label is not a statement about the
area.

Owed: either the area refines under its own target (and its own refusal or
bracket, per however the dial fork is ruled), or the result stops calling the
measurement converged where only the volume is. Read the fork's ruling first:
one designer's recommendation makes the bracket the primary value, which would
settle the label half.

## Re-checked on main `9267d3ae` (QUAD tracker sweep, 2026-10-27)

Still holds. The fork this row waited on,
`quad2-rational-max-rounds-dial-decision`, is closed, and the measure
door answered it with a bracket (PR 5741, `MassMeasurement`). The
area half stands. The patch lanes still take their area once at
`AREA_CELLS_PER_SPAN` (`crates/geom-brep/src/props/quad/area.rs`;
`rational.rs:457`, `patch.rs:266`), before and outside the round loop.
The label half is now `Stop::Met`, "every face met the resolution"
(`crates/topo/src/props/settle.rs:99`). The resolution is compared on
the flux alone (`mean_boundary_displacement`,
`crates/topo/src/props/quad_lane.rs:513`), so `Met` still says nothing
about the area beside it.
