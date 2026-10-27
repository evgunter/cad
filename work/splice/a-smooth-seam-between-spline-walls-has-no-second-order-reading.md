---
id: a-smooth-seam-between-spline-walls-has-no-second-order-reading
kind: issue
title: A seam between two spline walls that reads smooth refuses at the must-carry rule, whose second-order jet and arm read implicit forms
status: open
opened: 2026-10-27
priority: P2
cost: M
---


Filed by the SPLICE lane that gave the result's seam a spline reading
(`the-results-seam-between-two-spline-walls-is-read-by-no-dihedral`).

## What

`topo::boolean::ops::describe_edges` now reads a seam between two
spline faces through `geom_brep::classify_seam`, so such a seam
classifies `Transverse` or `Smooth`. A `Smooth` one goes on to the D6
smooth ladder, `must_carry_reading` → `geom_brep::must_carry_over_edge`,
which re-reads the first order through `classify_dihedral` at the
edge's interior stations and the second order through
`tangent_second_order` (`folded_lever_arm`, `crate::tangent_jet`). Both
read implicit forms, poison on a spline surface, so the seam refuses
typed (`DihedralReading::Lever(Arm)`, an invalid margin) rather than
reading which description it must carry.

The unit row `dihedral::seam_rows::two_spline_walls_read_transverse_or_smooth`
shows the first order reading `Smooth` on such a pair; nothing past it
reads.

## The likely shape

The stations' chart parameters off each face's pcurve, as
`ops::seam_face` reads the witness's; the first order through
`classify_seam`; the second order's relative normal curvature from the
two charts' jets at those parameters, levered over the same arm the first
order reads (`NurbsLane::chart_arm`, the chart's own length scale at the
station). Not a whole-chart curvature bound: a shorter arm shrinks the
sagitta margin toward Zero, which here reads a determinate seam as
under-determined.
