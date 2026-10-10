---
id: plane-cylinder-gap-reads-the-3d-distance-not-the-in-section-stand-off
kind: issue
title: pc_parallel_gap reads the 3-D axis-to-plane distance, not the wall's stand-off in the section
status: open
opened: 2026-10-07
priority: P3
cost: M
---

## What

PR 4280's review note (d): `pc_parallel_gap`'s datum `r − |gap|`
(`plane_cylinder_ruled`, `crates/geom-brep/src/intersect.rs`) reads the
axis' 3-D distance from the plane at the foot. The wall's stand-off in
the cross-section is `r·cos β − |gap|`, so the datum over-states it by
`r·(1 − cos β)`, second order in the tilt. With the tilt summed beside
it that term is what moved
`section_reads_at_the_reach::a_tangent_cut_turns_about_its_rulings_hinge_not_its_foot`
from `c² = 0.8·ε` to `0.6·ε`. Reading the in-section stand-off needs
the tilt's lever to reach from the foot (`|shift|` past the hinge's
lever) so the sum still bounds the stand-off across the reach, and a
check that the served set stays inside main's
(`r(1 − cos β)` covered by the swing).

## The shape of a fix

Read `r·cos β − |gap|` at the foot, lever the tilt from the foot, and
restore the row's `0.8·ε` pose if the hinge-versus-foot argument still
separates there.
