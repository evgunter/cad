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

**Both lanes now.** The splitting lane's in-plane conic departure
(`splitting/neighborhood.rs`, `split_conic_departure`'s Zero) reads
through the same `geom_brep::conic_bend_side`
(`two-second-order-conic-readers-certify-one-fact-differently`), so a
spiric edge departing in a split plane refuses `SliverSector` on
`conic_bend` there too, where it used to read its curvature at the
vertex as a bare sagitta. A cure lands once, in
`crates/geom-brep/src/conic_bend.rs`. No corpus test reached that tie
in either lane (probe over the split-touching sweep, editor-core and
topo suites, 2026-10-12: every tie read was a circle or a fixture's
ellipse).
