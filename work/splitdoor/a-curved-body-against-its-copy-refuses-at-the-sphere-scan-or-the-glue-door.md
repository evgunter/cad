---
id: a-curved-body-against-its-copy-refuses-at-the-sphere-scan-or-the-glue-door
kind: issue
title: A ball, a dome or a revolve with a cone face, booleaned with an identical copy, refuses before the On verdict: at the sphere extent scan or the glue door's second order
status: dispatched
opened: 2026-10-28
priority: P1
cost: M
---


Found by the sweep on `splitdoor/two-rims-coplanar` (the class: a curved body against a coincident copy, where nothing crosses and each shell lies on the other).

That branch lets the no-crossings section pass defer a refusing pair the reduction settled one carrier to the fallback's shell verdicts. The pair is answered where both faces' shells read `On` each other (`boolean::ops` `section_extent_pass`, `OnDeferred::answered`). A rod, a cut rod and a full torus against an identical copy now build: `∪` and `∩` the body, `∖` nothing. Measured at ε = 1e-9, every op in both orders, with the `sweep::test_support` fixtures, each against itself:
- **`ball_poled_z(1, 0)`** refuses `SpheresMeet { operand: A, face: FaceKey(1v1), verdict: Zero(Classified { margin: 0 }) }`. That is the sphere extent scan (`ops::sphere_extent_scan`), which runs before the section pass and has no deferral to the `On` verdict.
- **`dome(1)`** refuses `FallbackExtentUnsupported { face: FaceKey(5v1), what: "the sphere's section circle runs near the plane face's boundary — whole-circle membership cannot be certified …" }`, from the same scan.
- **`waisted`, `lantern`, and a revolved cone frustum** (`(0,0) (1,0) (0.5,1) (0,1)` about `y`) refuse `Escalated { decision: Coincidence(Contact, Moot), predicate: "contact_tangent_second_order", margin: Invalid }`. That is the glue door's tangent lane, before the reduction. The margin's `Invalid` is unmeasured past that.

Under the brick's, the rod's and the torus's reading, each should be one body. The sphere scan's case is the deferral above, applied to a sphere pair the reduction settled one carrier. The glue door's case needs its own measurement first: which face pair it reads, and why the second-order margin is `Invalid` on a pair the first rung should have read one carrier.
