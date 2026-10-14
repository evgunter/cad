---
id: chord-join-cone-lane-levers-from-the-base-vertex-not-the-apex
kind: issue
title: chord_join's cone lane levers plane_cone_section at the face extent from the base vertex, not the face's distance from the apex
status: open
opened: 2026-10-07
priority: P3
cost: M
---


Found by the sweep of the measured-levers lane
(`tang/measured-levers-reach-the-region`), which moved chord_join's
cylinder lane to the wall's axial extent beside it.

## What

`chord_join::section_case` (`crates/topo/src/chord_join.rs`) hands
`geom_brep::plane_cone_section` the scalar `extent` from
`section_reach`: for a cone, `splitting::rules::face_extent`, the
farthest boundary point of the face from the BASE VERTEX. The cone's
angular rows (`pn_aperture_sin`, `pn_aperture_cos`, `pn_apex_section`,
`pn_conic_type`) meter an angle whose displacement grows with the
distance from the APEX, and `route_pose` (`geom_brep::intersect`)
hands the same arm its edge's lever from the apex
(`reach.lever_from(apex)`). On a frustum face far from its apex, the
face extent from a vertex is shorter than the face's distance from the
apex by up to that distance, so a near-parabola (`pn_conic_type` in
the band at the apex distance) can read definite and be served as an
ellipse or refused as a hyperbola. Not measured yet; the rows to build
are a short frustum face 10 m from its apex with a conic-type
discriminant in the band there.

## The shape of a fix

Lever the cone lane at the face's farthest distance from the apex (the
`boundary_reach` walk in `splitting/rules.rs` with the apex as its
point), after confirming in `plane_cone_section`'s docs that the apex
is the pivot its `extent` is measured from, as PR 4231 stated for
`cone_cylinder_section`.
