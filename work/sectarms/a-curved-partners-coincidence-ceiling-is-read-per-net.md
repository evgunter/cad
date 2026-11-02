---
id: a-curved-partners-coincidence-ceiling-is-read-per-net
kind: issue
title: A curved partner's coincidence ceiling takes the net's largest |f| and least radial reach together, about 5.5 times loose on a torus wall spanning both equators
status: open
opened: 2026-11-02
priority: P3
cost: E
refs: [a-nurbs-face-against-a-cone-or-sphere-has-no-section-arm, 6304]
---


Filed by the NURBS × implicit arm's stage 1 (PR 6304), from its dual
review.

## What

A curved partner's hull decides coincidence on a ceiling in metres
(`pair_section/spline_implicit/curved.rs`, `hull`;
`ssi/residual.rs`, `Curved::ceiling_metres`). The ceiling takes the
net's largest `|f|` over all cells and its least radial reach over the
whole net's hull together. On a torus wall spanning both equators, the
largest `|f|` sits at the outer equator and the least reach at the
inner, so the ceiling is about 5.5 times the distance.

This is conservative: a wall within the band reads in band (escalates)
rather than coincident. It is never unsound. One consequence is that a
torus wall a few bands off its torus is not decided by the hull, and its
pass and sweep spend the cell budget. The row
`a_torus_wall_three_bands_off_its_torus_is_not_on_it` reads the hull
alone for that reason.

## Closing it

Take the ceiling per Bézier cell, each cell's `|f|` against its own
box's least reach, and fold the cells' ceilings by their maximum.
