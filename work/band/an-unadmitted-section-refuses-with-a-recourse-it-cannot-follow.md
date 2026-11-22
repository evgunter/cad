---
id: an-unadmitted-section-refuses-with-a-recourse-it-cannot-follow
kind: issue
title: A junction section that does not admit at the scalar refuses with the geometry recourse, which a plane-and-cone outline cannot follow
status: open
opened: 2026-11-17
priority: P3
cost: E
---

Found in PR 7050's review (the band-stripfru lane's fix pass files it).

A junction's section that does not certify as the shared row of its two bands refuses `UnsupportedGeometry` ("a junction's section does not certify as the shared row of its two bands", `surgery::attach_contact`). The variant's recourse is the shared `FILLET3_GEOMETRY_RECOURSE`: "the blend reads only planes, spheres, cylinders and cones whose edges are lines and circles … cut a feature that leaves any other ring AFTER the blend". For a chamfered rounded plate at `Interval` at ε = 1e-12, every face is a plane, a cylinder or a cone, and every edge is a line or a circle, so the sentence names nothing the user can change. The fillet's mixed rims at Interval at ε 1e-12 reach the same refusal where their sections run off the axes.

The refusal is a capability gap of the certificate at the scalar, not a geometry the blend cannot read. It wants either its own variant or detail with a recourse that is true of it (another scalar, another tolerance, or none), within the 75-word budget. The recourse text is shared by every `UnsupportedGeometry` site, which is wider than PR 7050.
