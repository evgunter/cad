---
id: the-lower-malign-arm-is-unwitnessed
kind: issue
title: nurbs_cert::malign's lower arm (residual below 1/SAFE_ASPECT) is unmeasured, unwitnessed by any body, and snaps columns against an off-row hazard
status: open
opened: 2026-10-21
priority: P3
cost: E
---


## Finding

From #5531's FULL review (MINOR-2). `nurbs_cert::malign` judges a band's
residual aspect `a_b / metric` two-sided: malign beyond `SAFE_ASPECT` (5) or
below `1/SAFE_ASPECT`. The ruling on `lofted-rational-walls-refuse-at-some-deltas`
asked for the two-sided form, and #5531 keeps it. Three things about the lower
arm are open:

- **Unmeasured.** `SAFE_ASPECT`'s measurement (dual review of PR #594, the
  tour's certificates at 5.0) was taken one-sided and in raw UV, on the upper
  arm only. The lower arm takes `1/SAFE_ASPECT` from the symmetry of the
  sliver certificate `(aspect² + 1)/8 · δ_s`. That symmetry holds for the
  formula, but no number was taken there.
- **Unwitnessed.** No corpus body has a band below `1/SAFE_ASPECT` over its
  metric. The reviewer's one-sided mutant (the lower arm deleted) is green on
  every row except the two `nurbs_cert` unit rows that state the arm
  (`band_schedule_snaps_on_realized_aspect`'s band 2,
  `malign_is_two_sided_and_relative_to_the_metric`).
- **Remedy aimed at the other hazard.** The snap projects a malign band onto
  the patch column count. That fixes the upper arm's hazard: off-column
  points beside wide columns. A band whose columns are too narrow for its
  rows has an off-ROW hazard: points between rows, like a seam chord point
  between two band rows. More aligned columns do not put those points on a
  row.

## To do

1. **Witness or retire.** Build a face that has a band below `1/SAFE_ASPECT`
   over its metric. The metric is a geometric mean of the extremes, so this
   needs a face whose band aspects span more than 25:1. Mesh it at the
   review's twenty δ with the falsifier armed, with and without the lower
   arm.
   - If the arm's snap is what keeps it under δ, keep the arm, cite the
     measurement in `SAFE_ASPECT`'s docs, and pin the body.
   - If the face meshes either way, or the snap does not help, retire the arm
     or give it an off-row remedy (rows through boundary points: direction 1
     of the closed item).
2. **Meter.** `FaceMeasure` does not carry the lattice metric, so the
   residual `realized_aspect / metric` cannot be recomputed from the budget
   CSV. That leaves the witness plan's seed row with nothing to read. Adding
   a `metric` column to `FaceMeasure`, the tess-meter CSV and tess-lint's
   reader is part of this item: three crates and a baseline column.
