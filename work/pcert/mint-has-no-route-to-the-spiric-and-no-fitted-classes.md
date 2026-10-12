---
id: mint-has-no-route-to-the-spiric-and-no-fitted-classes
kind: issue
title: the mint has no route to two minor uncovered classes, so their faces stay uncached under C4's exemption
status: open
opened: 2026-10-02
priority: P2
---

Filed by PCERT's `pcert/at-rest-rows-mandatory` (PR 3759), which makes
pcurve rows mandatory at rest (C4). C4 says every class of carrier a
chart can hold has a route into a certified row, and a face no route
covers refuses at the producer. Until each class's route lands, the
mint and tier 3 excuse it through one predicate
(`crates/topo/src/pcurves.rs`, `not_owed`), on
`PcurveCertifyError::UnsupportedCarrier` with its `UncoveredClass`.

This row is the schedule for `UncoveredClass::MirrorTorusSpiric, NoFittedClass`: two: a spiric on its torus's mirror through the cutting plane; and a line, ellipse or spiric offered a fitted-grade image, for which no fitted class exists. (A third, the zero-offset spiric, left the roster when `Curve3::spiric` began refusing a plane through the axis: that oval is a `Circle` carrier, so no spiric is one.)

When the route lands, the class leaves `not_owed`'s first arm in the
same change, so a face of that class is minted or refused at the
producer.

## The NoFittedClass half (closed 2026-10-12)

`UncoveredClass::NoFittedClass` is deleted on
`pcert/fitted-row-reads-carrier-and-chart`. The fitted-grade row reads
its carrier and chart alone, so a line, ellipse or spiric on a spline
chart certifies its `General` image at the hull, or refuses there
typed. The mint's `Intersection` arm gives such a carrier its General
image after the exact candidates. What this row still schedules is the
spiric half.
