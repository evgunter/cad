---
id: a-seam-on-a-creased-spline-chart-reads-no-arm
kind: issue
title: A seam on a C0-creased spline chart reads no arm anywhere: the chart's certified curvature bound refuses the whole chart, not the crease's neighbourhood
status: open
opened: 2026-10-27
priority: P3
cost: M
---


Filed by the SPLICE lane that gave the result's seam a spline reading
(`the-results-seam-between-two-spline-walls-is-read-by-no-dihedral`).

## What

`geom_brep::classify_seam` levers a spline face's normal over `1/κ̄`,
`κ̄` a certified bound on every principal curvature anywhere on its
chart (`NurbsLane::chart_curvature`, from
`patch_bound::patch_cells_refined`). That door refuses a chart with an
interior knot of multiplicity at least the degree, so a seam on any
face of a C⁰-creased chart reads a poisoned arm and escalates
(`LeverArm(Seam)`, `"dihedral_arm"`, invalid), however far the seam is
from the crease. Unit row: `dihedral::seam_rows::a_creased_chart_reads_no_arm`.

That is sound: no curvature is read across the crease, and the arm
claims nothing for the surface past it. It is coarser than it needs to
be.

## The likely shape

Bound the curvature on the crease's sides separately
(`patch_bound::patch_cells_across_creases` reads each side's cells on
the rational arm) and cap the arm by a certified lower bound on the
point's distance to each crease line within reach, so the arm never
reaches across one. The integral arm has no across-crease cells today.
