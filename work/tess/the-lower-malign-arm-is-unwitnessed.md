---
id: the-lower-malign-arm-is-unwitnessed
kind: issue
title: the malign-band column snap is unwitnessed over the lattice metric (0 of 2573 tour bands since #5531), its lower arm unmeasured and aimed at the wrong hazard; witness it or retire it
status: closed
opened: 2026-10-21
priority: P3
cost: E
closed: 2026-10-21
pr: 5551
branch: tess/snap-witness-or-retire
---


## Widened (2026-10-21): the whole snap

Since #5531 the trimmed lane judges malignity over the face's lattice
metric, and across the tour the snap now fires on **0 bands**. On `main`
before #5531 it fired on 2573. So the **upper** arm may be dormant too, and
the question is the snap as a whole, not only its lower arm.

**Witness or retire.** Find any body where either arm fires over the
metric, and check whether the snap is what keeps it under δ. Mutate the
snap off and compare at the review's twenty δ with the falsifier armed.
- If no body needs it, the snap is dead code to retire. The typed
  `CertificateExceeded` stays as the backstop, and a row shows that a face
  which would have needed the snap refuses typed instead of meshing wrong.
- If a body needs it, that body is the witness row, per arm.

The meter's `snap_bands` column (tess-meter CSV, tess-lint, the baseline)
stays either way. On retirement it reads 0, and its removal goes to the
budget lane's re-cut.

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

## Resolution (2026-10-21): retired

Probed with two local mutants of `band_schedule`: one that logs every band
either arm marks malign over the metric, and one with the snap off.

- **Tour:** 108 band schedules (the budget sweep), 0 fire either arm. The
  worst residual is 0.41–1.88, against the [0.2, 5] window.
- **Mesh suites:** 754 schedules. Only the synthetic fixture built to fire it
  fires (residual 9.09). With the snap off, the full mesh suite, including
  the slow set and the armed z1 falsifier, fails only that fixture's row.
  `demos/tour`'s suite, the default-ε walk included, passes. (The 1e-6 walk
  fails identically with the snap on: projectbox wall 1, filed separately.)
- **Adversarial faces:** degree-2 lofts through stations at z = 0, e, 2e,
  0.5, 1 (e from 0.1 down to 0.001), as four-arc rounds, two-arc creased
  rounds and four-arc cylinders: 15 bodies, the review's twenty δ, falsifier
  armed, `check_mesh` on every mesh. **Both arms fire**, up to 16 upper and
  32 lower bands per schedule, residuals 0.032–30.3. Outcomes are identical
  with the snap on and off on every body and δ, and the worst ratio with it
  off is 0.4996. On 14 rows the snap changed counts, and there it saved up
  to 14% of the triangles
  (`work/budget/uneven-station-lofts-cost-triangles-without-the-column-snap`).
  The cylinders refuse typed at 20 fine-δ rows either way, a separate
  liveness class with the snap firing on 0 of their bands
  (`work/tess/cylinder-lofted-through-uneven-stations-refuses-at-fine-delta`).

So the snap kept no face sound, and it is retired with `SAFE_ASPECT`,
`malign` and `NurbsFaceBound::step_v_at`. The typed `CertificateExceeded` is
the backstop. The meter's `snap_bands` column reads 0
(`work/budget/snap-bands-column-reads-zero-since-the-snap-retired`).

This item's to-do 2 (the `FaceMeasure` metric column) is not done here; it is
scheduled as `work/budget/face-measure-carries-no-lattice-metric`.
