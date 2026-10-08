---
id: point-to-axis-rejection-is-spelled-by-hand-at-each-site
kind: issue
title: the point-to-axis rejection is spelled by hand at each classifier site, beside Vec3::reject_from
status: open
opened: 2026-10-07
priority: P4
cost: E
refs: [cylinder-offsets-read-at-a-stored-origin-off-the-reach]
---


## What

`geom_core::Vec3::reject_from` is the exported home of the rejection
`v − a(v·a)`, with its overflow and underflow bands documented. The
section classifiers and their neighbours spell it by hand instead:
`geom_brep::intersect::parallel_axes_at` (`d_vec`), `axes_feet`'s
callers, `plane_cylinder_ruled`, `topo::boolean::carrier_eq`'s private
`perpendicular`, `surface_group::off_axis`, `section_cert::axis_pose`,
`offset_axial`'s `Frame::radial`. `cone_cylinder_section`'s
`coc_coaxial` reads through `reject_from` since PR 4231.

Routing them through one spelling moves low bits of pinned margins, so it
is its own change with its own re-baselining. Grep `x - a * x.dot(a)` and
`perpendicular(` for the hit list.

Raised by PR 4231's review (S3).
