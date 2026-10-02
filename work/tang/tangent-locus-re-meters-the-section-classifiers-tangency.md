---
id: tangent-locus-re-meters-the-section-classifiers-tangency
kind: issue
title: tangent_locus's plane×cylinder and equal-cylinder arms re-decide the tangency the section classifiers already meter, under other predicate names
status: open
opened: 2026-10-01
priority: P1
cost: M
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
