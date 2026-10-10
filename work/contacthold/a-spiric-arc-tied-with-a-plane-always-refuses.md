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
a boolean vertex). A curvature bound for the spiric (it is the image of
the minor circle under a map whose derivative is at least 1, which
`geom_brep::edge_extent` already uses) would give it the circle's
reading.
