---
id: a-vertex-met-twice-on-a-spline-face-keeps-two-rounded-uv-locations
kind: issue
title: A vertex a trimmed spline face's loop meets twice keeps two UV locations its pcurves agree on only to rounding
status: open
opened: 2026-11-20
priority: P3
cost: M
---


Found by `tesslie/nu-2-repeat`'s sweep.

## What

`crates/mesh/src/trimmed.rs` reads each trim-polygon entry off its own
traversal's pcurve (`loop_polygon`), so a vertex the loop meets twice
gets two images that agree only to rounding. On an analytic chart,
`one_location_per_branch` now pins the later entry to the first one's
UV wherever `Period::near` puts it on the same branch, using
`Surface::angle_periods`. The CDT then holds the vertex as one handle,
which `Pinches` meshes. Before the pin, the cylinder bodies in
`a_cylinder_wall_through_a_prism_corner_meshes_at_a_coarse_delta` and
`curved_notch_pinch::the_curved_notch_less_a_corner_pinch_meshes`
panicked at the cylinder arm's `nu == 2` assert, or refused
`Triangulation`.

A spline chart (`Surface::Nurbs`, `Surface::Approx`) is read as walked.
`angle_periods` answers `None` there, and a closed spline's periods are
its knot domains', decided at a band (`topo`'s `chart_u_period`,
`chart_v_period`, both `pub(crate)` in `topo::pcurves::mint`). So such a
face keeps one mesh id at two CDT vertices an ulp apart.

Unmeasured: no spline-face body that passes a vertex twice has been
built. The analytic arm's measurement says what to expect: `Triangulation`
refusals and the duplicate-id drop at a sliver.

## The shape to give

Either have the spline arm decide its periods (a door for
`chart_u_period` and `chart_v_period`, at the run's band) and pin the
same way, or measure a spline witness first and show the walked
polygon meshes right.
