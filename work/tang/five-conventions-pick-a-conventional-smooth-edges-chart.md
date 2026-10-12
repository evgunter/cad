---
id: five-conventions-pick-a-conventional-smooth-edges-chart
kind: issue
title: five conventions decide which adjacent chart a conventional smooth edge rests in
status: open
opened: 2026-10-12
priority: P1
cost: M
design: true
---


## Finding

Raised by the review of the PR that routed `Body::split_edge`'s tangent
children through the must-carry rule.

A smooth edge the rule calls under-determined stores a chart image,
and the rule leaves "which adjacent chart" to the caller: it is the
`chart` argument of `geom_brep::must_carry_description`, handed back
in `MustCarryDescription::Conventional`. Each caller decides differently:

| caller | chart |
|---|---|
| `chord_join::chord_spec` (split's tangent ruling) | the section plane |
| `splitting::finish::describe_section_boundary` | the section face's surface |
| `sweep::extrude`, cap rim | the wall |
| `sweep::extrude`, strut | the previous segment's wall (`k_prev`) |
| `sweep::revolve::upgrade` | `s1` |
| `sweep::blend::surgery`, contact edge | `he_plus`'s face |
| `boolean::ops`, a stale curved seam | `s1` |
| `split::Body::tangent_piece` | `he_plus`'s face |

Each is argued sound (the locus lies exactly in both charts), and the
battery holds each one up. The cost is that one geometric seam is
stored two ways depending on the door that made it. Observed on
`wedge_end_doors.rs`'s shoulder: the seam a thin shoulder's cut mints
stores `Chart(Plane)` through the chord, while the same ruling cut
short by `split_edge` stores `Chart(Cylinder)`. A reader that compares
descriptions (naming, adjacency coherence, a later restatement) sees
two answers for one locus.

## The question

Whether the chart should stay each caller's choice or become a function
the rule computes, and if so which: a function of the pair alone (key
order, or a kind preference such as "the plane if one is a plane"), or
of the edge (its orientation). Today `MustCarryDescription::Conventional`
carries the chart, but only as the caller's `chart` argument to
`geom_brep::must_carry_description` handed back; a ruling either keeps
that argument or replaces it with the rule's own pick, after which each
caller above drops one argument. Weighed with designers first.
