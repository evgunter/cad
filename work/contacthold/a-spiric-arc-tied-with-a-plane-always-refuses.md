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
