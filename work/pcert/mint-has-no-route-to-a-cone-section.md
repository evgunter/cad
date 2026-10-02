---
id: mint-has-no-route-to-a-cone-section
kind: issue
title: the mint has no route to a tilted cone section, so a cone face bounded by one stays uncached under C4's exemption
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

This row is the schedule for `UncoveredClass::ConeSection`: a conic on a cone that is not a rim (a tilted plane section, an ellipse, or a circle tilted off a rim within the band). The cone has no fitted certificate, so no route exists yet. REACH's `plane-cone-elliptic-section-split-refusal` and CONTACT's `cone-chart-trim-reads-a-tilted-section-as-its-vertex-window` are neighbouring rows, about the split and the trim rather than the row.

When the route lands, the class leaves `not_owed`'s first arm in the
same change, so a face of that class is minted or refused at the
producer.
