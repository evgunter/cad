---
id: the-census-reads-a-span-against-a-faces-boundary-on-its-own-loop
kind: issue
title: census boundary_crossings reads an edge against a face's boundary on its own loop over curve_pair::meet, beside the curve × face table's boundary pass
status: open
opened: 2026-10-26
priority: P3
cost: E
refs: [a-span-is-read-against-a-faces-boundary-edges-first]
---



Filed by SPLICE's boundary-first lane (PR 5768), from its review (R1).

## What

`geom_brep::curve_face::boundary` is now the one reading of a span
against a face's boundary edges (C5's curve × face clause): each
boundary vertex on the span, then each boundary edge through
`curve_pair::meet`. The boolean reads through it
(`topo` `carrier_cross::read_boundary`, behind `boundary_crossing` and
`reduce::spline_face`).

`crates/topo/src/census.rs` `boundary_crossings` (with
`edge_crossings`) is a third reader of the same question on another
stack: each loop through `splitting::containment::carrier_loop`, each of
its edges against the census edge by its own `meet` call, with its own
refusal mapping. It is not known to be wrong. Two readings of one
question can disagree at the band's edge, and the census is the
at-rest check the boolean's records are compared against.

## Fix

Read the census's edge × face boundary through
`curve_face::boundary`, keeping the census's cut placement and its
refusal reporting, or say in the census why its reading must differ.
