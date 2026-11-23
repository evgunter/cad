---
id: a-seam-on-a-creased-spline-chart-reads-no-arm
kind: issue
title: A seam on an integral spline chart with a C0 crease anywhere reads no arm: the patch bound refuses the whole integral chart, where a rational one is read up to its crease
status: parked
opened: 2026-10-27
priority: P3
cost: M
blocked_on: [the-crease-arms-retire-once-no-carrier-carries-a-c0-knot]
---


Filed by the SPLICE lane that gave the result's seam a spline reading
(`the-results-seam-between-two-spline-walls-is-read-by-no-dihedral`).

## What

`geom_brep::classify_seam` levers a spline face's normal over the
chart's own length scale at the seam's point (`NurbsLane::chart_arm`,
`offset_meters::chart_arm`): the largest radius within the reach over
which the certified curvature of the cells it reaches stays at most its
reciprocal. The cells come from `patch_bound`. A rational chart's are
read across a C⁰ crease (`patch_cells_across_creases`, each cell one
side's), so a crease bounding a cell within the reach poisons the arm and
one past the reach does not (unit row
`dihedral::seam_rows::a_crease_within_the_reach_reads_no_arm`).

An integral chart's cells come from the integral arm, which refuses any
interior knot of multiplicity at least the degree
(`patch_bound::check_direction`). So a seam on a face of an integral
chart with a C⁰ crease anywhere reads a poisoned arm and escalates
(`LeverArm(Seam)`, `"dihedral_arm"`, invalid), however far the crease is.
Sound, since no curvature is read across the crease, but coarser than
the rational arm.

## The likely shape

Read the integral chart's cells per side of each crease, as the rational
arm reads its spans' Bernstein blocks (`patch_cells_across_creases`'
docs say why the integral arm, which differences the whole net first,
cannot today): split the net at its creases into C¹ pieces and read each.

Moved from SPLICE to SPLICESEAM on 2026-10-28, by `git mv` in SPLICE's split along its seams (Track size; Ev, in chat, 2026-10-28).

Moved from SPLICESEAM to SPLICE on 2026-11-02, by `git mv`, when SPLICESEAM merged back into SPLICE: the track budget now counts only the P0, P1 and unbanded rows a sitting dispatches (Ev, in chat, 2026-11-02), so the four tracks of SPLICE's 2026-10-28 split came in under one budget.

## Parked on the crease arms' retirement (SPLICE, 2026-11-03)

`tesssize/the-crease-arms-retire-once-no-carrier-carries-a-c0-knot` names this row as one that becomes unreachable under DESIGN [topology] (Ev, `[ev]` PR 5913: every carrier C¹ strictly inside its cell; no reader carries a crease arm), and closes it. Parked on it rather than built, as `a-creased-spline-gets-no-second-order-clearing` was.
