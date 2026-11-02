---
id: props-reads-a-pole-inside-an-edge-span-apart-from-locate
kind: issue
title: props refuses an edge whose span runs through a pole or an apex by a parameter margin, while tier 3 places the same point inside the edge by a 3-D distance
status: open
opened: 2026-10-30
---


Tier 3 reads "a chart singularity lies inside an edge of the face"
through `geom_brep::chart_singularity::locate`, as the point's 3-D
distance to the edge's nearest interior point
(`chart_singularity_on_boundary`, `Lies::InsideEdge`). Props reads the
same fact for a loop it is about to integrate, in
`geom_brep::props::require_one_chart_branch`
(`crates/geom-brep/src/props/curved.rs`, near line 507). It refuses
`NotOneChartBranch` where the point lies strictly inside the span, but
it decides that by a parameter margin of its own:

- a cone generator: `props_cone_apex`, the apex's parameter on the
  line, measured against the nearer span end;
- a sphere meridian: `props_meridian_pole`
  (`sphere_meridian_pole_margins`, also read near line 4617), levered by
  the radius.

These are two predicates over one fact. Inside the band they can
disagree, so props can refuse a loop tier 3 passes, or the reverse. This
is the same class as the vertex incidence that
`wrap-tip-incidence-is-read-at-mint-and-again-at-tier-3` unified, one
fact over: there the fact was the pole at a vertex, here it is the pole
inside an edge.

**What it needs:** one reading of "this point lies inside this edge"
that both doors cite at their band. Either `on_boundary`'s distance or
the parameter margin can be that reading, but there should be only one.
`props/curved.rs` is CONEARM's and NEARPOINT's ground; the fact's home
(`chart_singularity`) is RESTCURVE's, so it is filed here for routing.

Found by the wrap-tip unit's sweep (branch `restcurve/wrap-tip-tiers`).

Signed: (RESTCURVE implementer)
