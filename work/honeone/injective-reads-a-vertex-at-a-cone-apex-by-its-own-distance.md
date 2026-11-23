---
id: injective-reads-a-vertex-at-a-cone-apex-by-its-own-distance
kind: issue
title: The self-intersection door decides a cone piece's vertex at the apex by a reading of its own
status: closed
opened: 2026-10-30
closed: 2026-11-22
---


`topo::injective`'s `present_chart` (`crates/topo/src/injective.rs`,
near line 1515) reads a cone piece's vertex as a collapsed side of the
chart where `|p − apex|` decides Zero (`self_intersection_apex`). That
is the fact `geom_brep::chart_singularity::vertex_at_singular_point`
(`chart_singularity_at_vertex`) is the one reading of: the loop walk's
reset joint, tier 3's at-vertex placement and the wrap-tip rule read it.
The quantity is the same, but the decision site is the door's own, so
the door's census and any change to the one reading part ways.

**What it needs:** call the one reading against the cone's apex
(`singular_points`). No program owns `injective.rs`
(`work.py territory`), so it is filed here.

Found by the RESTCURVE wrap-tip unit's sweep (branch
`restcurve/wrap-tip-tiers`).

Signed: (RESTCURVE implementer)

Moved from `work/issues/` to `work/honeone/` on 2026-11-03, when the `work/issues/` bucket was retired (Ev's direction, in chat); id and body unchanged.

## A note to this row's owner (EMBED, lane pole, PR 7115, 2026-11-18)

PR 7115 does what this row asks; closing the row is yours.

`present_chart`'s apex block and its `self_intersection_apex` site are
gone. A cone's apex and a sphere's poles are read in one place,
`injective`'s `singular_ends`, by `vertex_at_singular_point` against
the cone's apex and `sphere_poles`; `face_extent` and `present_chart`
both read through it. The audit row and the recourse roster's entry
went with the site. One reading serves both: the quantity was the same
distance at the same band, under a second name. The apex is spelled
off the `Surface::Cone` arm, not through `singular_points`, as the
door holds a lifted `Surface` and no `ChartInverse`.

## Closed (2026-11-22)

Closed by PR 7115, as the note above says: the door calls the one
reading at the cone's apex, and its own site is retired.
