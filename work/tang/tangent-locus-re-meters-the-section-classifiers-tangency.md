---
id: tangent-locus-re-meters-the-section-classifiers-tangency
kind: issue
title: tangent_locus's plane×cylinder and equal-cylinder arms re-decide the tangency the section classifiers already meter, under other predicate names
status: closed
opened: 2026-10-01
priority: P1
cost: M
closed: 2026-10-03
branch: tang/tangent-locus-consumes-classifiers
---


## What

`tangent_locus` (the DEV-1 witness lane, `crates/topo/src/boolean/rest.rs`
at filing; the m9-3 lane is moving it) decides plane×cylinder and
equal-radius parallel-cylinder tangency with its own metered rows
(`tangent_locus_gap`, `tangent_locus_axis_parallel`, …).
`geom_brep::intersect`'s section classifiers already decide the same
facts: `plane_cylinder_section` → `PlaneCylinderSection::TangentLine`
(`pc_parallel_gap`), and `cylinder_cylinder_section` →
`EqualCylinderSection::TangentLine`. The coaxial cylinder×sphere
`TangentCircle` likewise exists in `cylinder_sphere_section`, with
`cs_declared_coaxial` and `cs_wall_reach` metered. That is one fact
decided twice, under two K-funnel names, which can disagree at band.

## The fix's shape

The witness lane consumes the classifiers' tangency variants instead
of re-deriving them. Found by the DEV-1 circle-arm design pair
(2026-10-01), reading only; nothing measured.

## Review tier

SINGLE, FULL: one Opus reviewer, correctness as well as style, because
changing which predicate decides tangency can shift verdicts at band.

## Closed

`tangent_locus` reads the section classifiers' rows. Plane×cylinder is
`plane_cylinder_section`'s axis-in-plane lane, run on the cylinder
re-based to the foot of the consumed extent. Parallel cylinders share
`cc_axes_parallel` and `cc_parallel_gap` with
`cylinder_cylinder_section`. The two disagreed at band in two places,
and the classifier side was right in the first, the witness side in the
second:

- An axis in band of the plane escalated on the witness's
  `tangent_locus_side`; the section decides the crossing.
- `cc_parallel_gap` read `2·r1 − d`, so a declared equal pair within
  band of tangency escalated in one operand order and was tangent in
  the other; it reads `r1 + r2 − d` now, the witness's margin.

The coaxial cylinder×sphere `TangentCircle` has no witness arm to
consume (the lane's own docs keep it out), so nothing re-decides it.
Filed: `plane-cylinder-section-reads-its-gap-at-the-stored-origin`.
