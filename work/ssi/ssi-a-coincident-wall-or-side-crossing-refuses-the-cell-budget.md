---
id: ssi-a-coincident-wall-or-side-crossing-refuses-the-cell-budget
kind: issue
title: ssi: a plane holding a whole wall, or a wall side the locus crosses, refuses CellBudget, whose ending names neither
status: open
opened: 2026-10-04
priority: P3
cost: M
---


(SSI investigation lane `ssi/dome-remeasure`, 2026-10-04, from
re-measuring `plane-nurbs-ssi-does-not-certify-a-curved-dome`.)

## What

Two plane × NURBS configurations of the curved dome `W(d)` refuse
`SsiError::CellBudget { budget: 200000 }` at every ε, in 0.3–0.4 s:

- **A whole wall in the plane:** the level plane `y = 0` and the flat
  wall `W(0)`. Every cell of the chart lies on the locus, so no
  enclosure excludes one and the subdivision spends its budget.
- **A locus crossing a coincident side:** the tilt `x + y = 0.5 − d/8`
  at d = 4 is `x + y = 0`, which holds the wall's whole `s = 0` side.
  The interior arc meets that side at `t(1−t) = 1/16`, where the
  locus crosses itself: `f = x + y = s·(1 − 16t(1−t)(1−s))` has a
  zero gradient there. (Measured with the window widened to 4 m; at
  2 m it refuses `WindowShortOfWall`, filed as
  `ssi-window-check-reads-the-walls-control-hull`.)

Both refusals are typed and loud. Their ending,
`CELL_BUDGET_RECOURSE` (`crates/geom-brep/src/ssi.rs`), names a
smaller domain or surfaces that "cross clearly or stay clearly apart",
which fits a near-tangency. It does not say that the wall lies in the
plane, or that the locus has a singular point on a side that lies in
the plane. Neither is a curve the lane can trace, so a refusal is the
right answer; the question is whether it should name the coincidence.

## Fix shape

Not designed. The boundary pass already certifies a side within ε of
the plane (`SideClass` in `ssi/boundary.rs`, the `Side` region of
`ssi-a-plane-through-a-faces-vertex-is-a-point-contact-not-a-refusal`),
so a pass that finds every side coincident, or a coincident side with
a root of the interior locus on it, has the fact in hand before the
subdivision runs. Whether to refuse there by name, or to report the
whole-wall case as a region, is a design question.
