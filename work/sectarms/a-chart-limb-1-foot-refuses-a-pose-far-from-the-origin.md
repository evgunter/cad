---
id: a-chart-limb-1-foot-refuses-a-pose-far-from-the-origin
kind: issue
title: limb 1's foot on a NURBS chart refuses a pose a thousand metres from the origin, its projector's acceptance an absolute 1e-13 m
status: open
priority: P3
cost: M
opened: 2026-11-03
---

Found by PR 6412's dual review (R2), on the 1×2 lane that PR built.

## What happens

The sphere-and-strip scene of `crates/geom-brep/tests/implicit_chart_section.rs`
(`a_sphere_cuts_the_strip_in_two_arcs_across_it`), translated by
`(10³, −10³, 500)`, refuses at ε = 1e-9 with
`SsiError::FootPointInconclusive { t: 0.875, last_distance: 1.14e-13 }`.
Offsets of 10, 100 and 300 certify. The same strip translated against a
plane gives two branches (`Ok`).

## Why

Limb 1 checks each sample's foot on the NURBS chart by projection
(`crates/geom-brep/src/ssi/certify.rs`, `nurbs_limbs`, about line 740:
`surface.project_from_seed(..)`, mapped to `FootPointInconclusive`). The
projector accepts a foot within `PROJECT_EPS_POINT = 1e-13` m
(`crates/geom/src/projection_policy.rs:137`), an absolute distance. One
float spacing near 10³ is 1.1e-13, so a point a thousand metres from the
origin cannot be resolved that finely, and the projection does not
converge. The plane lane reaches the same limb only with a pcurve whose
foot it already holds, which is why the translated plane pose passes.

## What closes it

The projector's acceptance read relative to the coordinates' resolution
where the point lies (or limb 1 reading the foot's distance through a
door that states its own floor), with a row at a far offset. The
`geom` file has no owning program; this row sits on SECTARMS because the
1×2 lane is what reaches it.
