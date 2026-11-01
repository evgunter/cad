---
id: a-curved-body-against-its-copy-refuses-at-the-sphere-scan-or-the-glue-door
kind: issue
title: A ball, a dome or a revolve with a cone face, booleaned with an identical copy, refuses before the On verdict: at the sphere extent scan or the glue door's second order
status: closed
opened: 2026-10-28
priority: P1
cost: M
closed: 2026-10-30
pr: 6029
---


Found by the sweep on `splitdoor/two-rims-coplanar` (the class: a curved body against a coincident copy, where nothing crosses and each shell lies on the other).

That branch lets the no-crossings section pass defer a refusing pair the reduction settled one carrier to the fallback's shell verdicts. The pair is answered where both faces' shells read `On` each other (`boolean::ops` `section_extent_pass`, `OnDeferred::answered`). A rod, a cut rod and a full torus against an identical copy now build: `∪` and `∩` the body, `∖` nothing. Measured at ε = 1e-9, every op in both orders, with the `sweep::test_support` fixtures, each against itself:
- **`ball_poled_z(1, 0)`** refuses `SpheresMeet { operand: A, face: FaceKey(1v1), verdict: Zero(Classified { margin: 0 }) }`. That is the sphere extent scan (`ops::sphere_extent_scan`), which runs before the section pass and has no deferral to the `On` verdict.
- **`dome(1)`** refuses `FallbackExtentUnsupported { face: FaceKey(5v1), what: "the sphere's section circle runs near the plane face's boundary — whole-circle membership cannot be certified …" }`, from the same scan.
- **`waisted`, `lantern`, and a revolved cone frustum** (`(0,0) (1,0) (0.5,1) (0,1)` about `y`) refuse `Escalated { decision: Coincidence(Contact, Moot), predicate: "contact_tangent_second_order", margin: Invalid }`. That is the glue door's tangent lane, before the reduction. The margin's `Invalid` is unmeasured past that.

Under the brick's, the rod's and the torus's reading, each should be one body. The sphere scan's case is the deferral above, applied to a sphere pair the reduction settled one carrier. The glue door's case needs its own measurement first: which face pair it reads, and why the second-order margin is `Invalid` on a pair the first rung should have read one carrier.

## Closed (2026-10-30, PR 6029)

A ball, a dome, a lantern and a cone frustum against an identical copy now build: ∪ and ∩ give the body, ∖ gives `Empty`, at the closed form and tier 3′ at three ε. Four changes do it:
- `ops::sphere_extent_scan` defers a pair the reduction settles one carrier to the `On` verdict;
- the no-crossings certificates read the sweep's touches as events (W4, its soundness step written at `section_cert`);
- `section_extent_pass` walks every pair;
- a cone arm joins the carrier ladder.

One `no_crossings_pass` runs the scan, the re-cut check and the section pass for production and its test door alike. `carrier_eq::INVENTORY` is the one home for the ladder's kinds.

The waisted copy's waist edge still refuses `CurvedPierceUnsupported` (`Unsettled::Degenerate`, `bool_line_cone_lead`); that is `roots/a-line-parallel-to-a-cone-generator-has-no-root-lane`, pinned by cause.

FULL review: APPROVE WITH FIXES (0 MAJOR). Its probes covered near-copies offset within the band, scaled and turned copies, three operands and a two-sphere lens; none built wrong. A delta review gave APPROVE: production unchanged over about 6,170 rows and 115 slow rows.

Filed `no-fixture-reaches-a-settled-refusal-before-an-unsettled-one-in-the-section-pass` (P3).
