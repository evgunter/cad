---
id: mint-has-no-route-from-the-closed-form-door-to-a-spline-carrier
kind: issue
title: the mint has no route to a spline carrier on an analytic chart, so such a face stays uncached under C4's exemption
status: open
opened: 2026-10-02
---

Filed by PCERT's `pcert/at-rest-rows-mandatory` (PR 3759), which makes
pcurve rows mandatory at rest (C4). C4 says every class of carrier a
chart can hold has a route into a certified row, and a face no route
covers refuses at the producer. Until each class's route lands, the
mint and tier 3 excuse it through one predicate
(`crates/topo/src/pcurves.rs`, `not_owed`), on
`PcurveCertifyError::UnsupportedCarrier` with its `UncoveredClass`.

This row is the schedule for `UncoveredClass::SplineCarrier`: a spline carrier at the closed-form door (`chart_pcurve`) on an analytic chart. It has no `{1, cos, sin, t}` form, and the fitted lane that certifies one is reached by no mint site from that door.

When the route lands, the class leaves `not_owed`'s first arm in the
same change, so a face of that class is minted or refused at the
producer.
