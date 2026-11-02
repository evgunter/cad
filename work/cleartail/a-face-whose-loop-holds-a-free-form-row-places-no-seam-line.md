---
id: a-face-whose-loop-holds-a-free-form-row-places-no-seam-line
kind: issue
title: A face whose outer loop holds a row with no closed form has no extent, so the injectivity check places none of its seam lines and refuses beside them at any depth
status: open
opened: 2026-10-29
priority: P3
cost: M
---

## Finding

From CORNER (`departure-reads-no-curved-corner`).

`clearance::extent_of` builds a face's chart hull (`Boundary::extent`) from its outer loop's rows through `row_of`:
- a monotone coordinate line by its ends;
- a harmonic image by its enclosure over its span (CORNER).

Any other row — a fitted or projected spline pcurve — makes `row_of` answer `None`, and the extent is unknown. Then `injective::sheets` leaves the periodic axis's sheet `Sheet::Unknown`. `placed` puts no line on it, and `present_chart` (and `present_line_corner`) refuse `Structural` for every coordinate line of that face on that axis, at any depth. So a cylinder wall half whose loop holds one free-form row presents none of its seam lines. A pair of it against a cap, or against the other half, refuses at once if the partner refuses too.

Before CORNER this was the fate of every wall half a planar cut leaves (its ellipse arc is a harmonic row). That case is fixed; the free-form one stays.

## What a fix needs

An enclosure of the row over its span: a spline pcurve's net hull is one. `row_of` would then answer `Row::Bulging` for it as for a harmonic, under the same whole-turn guard.
