---
id: the-guards-listing-of-a-held-spline-pair-off-a-seam-or-pole-has-no-row
kind: issue
title: The guard's reading of a held spline x implicit pair whose reaching side does not bound its face has no row
status: open
opened: 2026-11-12
priority: P2
cost: M
---

Filed by the `sectarms/join-reads-spline-implicit` lane's fix pass.

## What

The interior-loop guard reads a spline × implicit pair the section
table holds off the pair's listing (`ops::pair_verdict`, through
`section_cert::of_listed`, the reading `section_cert::spline_implicit`
takes of an unheld pair). The listing differs from the carry in one
branch only: a classification that reaches
(`geom_brep::Components::Reaching`) whose sides do not all bound their
face (`crate::pcurves::chart_side_bounds`), which `of_pair_section`
hands to its marched reader. There the whole reading lists each rim
arc with `reaches: None` and `unbounded: false`
(`section_cert::of_pair_section`, the `Component` it builds per listed
component), where the carry would give `unbounded: true`, which W1
clears silently (`section_cert::of_carried`).

A side does not bound its face where the wall's chart closes in that
channel (a seam) or the side collapses to a point (a pole); the reading
is the surface's alone. No fixture in the
tree builds such a spline wall against an implicit partner through a
boolean: the loft builds one wall per profile segment, each bounded on
all four sides (`sweep::test_support::lofted_cylinder` is two
half-walls). So the branch runs on no row: the mutant that reads the
held pair's carry instead of its listing survives every row.

## Done when

A row builds a boolean whose guard reads a held spline × implicit pair
across a seam or pole of the wall, and goes red against the guard
reading the carry.
