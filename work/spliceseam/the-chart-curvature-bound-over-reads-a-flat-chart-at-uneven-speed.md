---
id: the-chart-curvature-bound-over-reads-a-flat-chart-at-uneven-speed
kind: issue
title: offset_meters' certified curvature bound reads a flat spline chart at uneven speed as curved (κ ≤ 266 on a plane), so the seam's lower arm collapses and the seam escalates
status: open
opened: 2026-10-27
priority: P3
cost: M
---


Filed by the SPLICE lane that gave the result's seam a spline reading
(`the-results-seam-between-two-spline-walls-is-read-by-no-dihedral`),
from the re-review of PR 5868. The meter is `crates/geom-brep/src/offset_meters.rs`
(`cell_curvature`, meter 2), territory of ENCL, OFFSET and SHELL; it is
filed here because the seam's arm is where its slack costs.

## What

`offset_meters::cell_curvature` bounds a cell's principal curvatures by
dividing the second fundamental form's interval enclosure by the
first's (closed form and Gershgorin, intersected). On a chart that is
exactly planar but runs at uneven speed, the enclosures of `S_uu`,
`S_vv` are wide and nonzero and do not cancel against the normal, so the
bound reads curvature where there is none. Measured by the re-review
(`dihedral::seam_rows::a_flat_wall_with_uneven_speed_never_reads_smooth`):

- a cubic wall, exactly planar, control rows `0, 0.01, 0.02, 1` along
  its profile, tilted 0.3 rad: the bound reads `κ ≤ 266`, and
  `offset_meters::chart_arm` gives `0.00375` where the true length scale
  is the unit reach;
- rows `0, 0.001, 0.002, 1`: an arm of about `1e-5`.

Under `classify_seam`'s bracket this costs only escalations at the arm
(`LeverArm(Seam)`, `"dihedral_arm"`), never a class. The offset meters
read the same bound for their collapse headroom, where it refuses a
regular patch for a fold it does not have.

## The likely shape

Bound the normal curvature from the normal's own change, `‖∂n‖ / speed`
per direction, with the normal's enclosure narrowed first, rather than
`II/I` assembled from separately enclosed forms; or refine the cell
schedule where a cell's speed spread is wide.

Moved from SPLICE to SPLICESEAM on 2026-10-28, by `git mv` in SPLICE's split along its seams (Track size; Ev, in chat, 2026-10-28).
