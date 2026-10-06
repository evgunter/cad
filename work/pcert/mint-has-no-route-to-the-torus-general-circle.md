---
id: mint-has-no-route-to-the-torus-general-circle
kind: issue
title: the mint has no route to a torus general circle, so a torus face bounded by one stays uncached under C4's exemption
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

This row is the schedule for `UncoveredClass::TorusGeneralCircle`: a circle on a torus that is neither a parallel nor a meridian (a Villarceau circle, or a circle perpendicular to the axis centred off it). The torus has no fitted certificate, so no route exists yet; an incidence test that tells such a circle from one off the torus is `work/issues/uncovered-chart-classes-have-no-incidence-test`.

When the route lands, the class leaves `not_owed`'s first arm in the
same change, so a face of that class is minted or refused at the
producer.
