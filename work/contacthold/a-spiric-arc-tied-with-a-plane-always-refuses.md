---
id: a-spiric-arc-tied-with-a-plane-always-refuses
kind: issue
title: A spiric arc tied with a plane face always refuses ConicBend, even lying in the face's plane
status: open
opened: 2026-10-10
priority: P4
cost: M
refs: [boolean-conic-side-code-zero-is-first-order, 4575]
---


## Finding

From the dual review of PR 4575. `Bend::of` gives a spiric carrier
`BendBounds::Unbounded`, so `sectors::conic_side` takes its lift as zero
and refuses `ConicBend` at every first-order tie with a plane face. That
includes a spiric arc lying in the face's plane, which a height bound
would read `On`, and one that clearly bends off. No modelled body is
known to reach it (a torus's plane section tangent to a partner plane at
a boolean vertex). It has no simple cure: `geom_brep::edge_extent`'s
map with derivative at least 1 bounds distances, not curvature, and a
spiric section can inflect, so it has no positive lower curvature bound
to give it the circle's reading. A cure reads the section's departure
some other way (e.g. a height bound for the in-plane case alone).

**The split cannot reach it.** The splitting lane reads an in-plane
conic departure through the same `geom_brep::conic_bend_side`
(`splitting/neighborhood.rs`, `split_conic_departure`'s Zero), but no
spiric edge gets there: the gate (`splitting/classify.rs`) refuses
`CurvedEdgeUnsupported` for a spiric edge unless `edge_clears` puts its
whole locus, or a face of it, off the plane's band, and `sector_face`'s
`Torus` arm refuses `CurvedBooleanUnsupported` behind it. The finding is
the boolean's alone; a cure lands once, in
`crates/geom-brep/src/conic_bend.rs`.

## The edge extent's new home (OFFSET, 2026-10-27)

`geom_brep::edge_extent` no longer exists (OFFSET, PR 5875). Its home is `geom_brep::extent`, beside the reach family, as three functions: `edge_extent_inf` (≤ the edge's diameter `D`: the chord, the minor-axis folds and the sampled pairs; the lever of a reading whose Zero only refuses), `edge_extent_sup` (≥ `D`; a reading whose Zero is acted on and whose definite signs only refuse) and `edge_reach_from` (the edge's farthest distance from a point of it; a reading that acts on every sign, read from that point). `ConicDeparture`, which this row's citation reads, levers at `edge_extent_inf`, which now also reads the spiric's sampled pairs.
