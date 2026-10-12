---
id: mint-has-no-route-to-a-cone-section
kind: issue
title: the mint has no route to a circle tilted off a cone's rim, so a cone face bounded by one stays uncached under C4's exemption
status: closed
opened: 2026-10-02
priority: P1
closed: 2026-10-07
pr: 4227
---

Filed by PCERT's `pcert/at-rest-rows-mandatory` (PR 3759), which makes
pcurve rows mandatory at rest (C4). C4 says every class of carrier a
chart can hold has a route into a certified row, and a face no route
covers refuses at the producer. Until each class's route lands, the
mint and tier 3 excuse it through one predicate
(`crates/topo/src/pcurves.rs`, `not_owed`), on
`PcurveCertifyError::UnsupportedCarrier` with its `UncoveredClass`.

This row is the schedule for `UncoveredClass::ConeSection`: a circle on a cone that is not a rim (tilted off one within the band). An ellipse carrier has its exact section image (`Pcurve::ConeSection`); the tilted circle is azimuth-non-harmonic, the cone has no fitted certificate, and no route exists yet. REACH's `plane-cone-elliptic-section-split-refusal` and CONTACT's `cone-chart-trim-reads-a-tilted-section-as-its-vertex-window` are neighbouring rows, about the split and the trim rather than the row.

When the route lands, the class leaves `not_owed`'s first arm in the
same change, so a face of that class is minted or refused at the
producer.

Closed by `pcert/torus-villarceau-route` (PR 4227): the class is deleted. A circle on the chart that the incidence test reads on it is imaged (`Pcurve::FocalSection`: a Villarceau circle on a torus, a tilted section ellipse on a cone) or refuses `CarrierGrazesChart`, and one read off it refuses `CarrierOffChart`; neither is excused.
