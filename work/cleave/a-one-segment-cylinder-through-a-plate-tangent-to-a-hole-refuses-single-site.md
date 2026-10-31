---
id: a-one-segment-cylinder-through-a-plate-tangent-to-a-hole-refuses-single-site
kind: issue
title: A one-segment cylinder through a plate, a hole tangent to its wall, refuses SingleSiteSectionLoop{count: 2} in every op where the two-arc form builds
status: dispatched
opened: 2026-10-27
priority: P1
cost: M
branch: cleave/one-segment-cylinder-through-a-holed-plate
---

Found by PR 5808's claims review, in the class of
`one-segment-tangent-rims-refuse-where-their-two-arc-forms-build`.

## The pose

A cylinder `r = 0.3`, `z ∈ [−1, 1]`, through the plate
`[−1, 1]² × [−0.5, 0]` holed through by `r = 0.1` tangent to the
cylinder's wall along the ruling through `(0.3, 0)`, outside it (about
`(0.4, 0)`) or inside it (about `(0.2, 0)`). With both vertices at the tangent azimuths, or the
cylinder's alone, the one-segment form refuses
`Join(SingleSiteSectionLoop { count: 2 })` in every op, while the
two-arc form builds; the same on `main` (the review's probe, `S4`
scenes).

## Why, as far as read

The cylinder crosses the plate's top and bottom faces, so each cut is a
one-site section loop at its rim's vertex, and the hole's wall is tangent
to the cylinder's along the ruling through both sites: the loops meet
an edge of another face there (`boolean/join.rs` `site_cell` reads
`SiteCell::Other` against the wall's rim on each side), which no arm of
`closes_at_its_site` takes for a transverse section.
