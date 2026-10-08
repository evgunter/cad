---
id: mint-has-no-route-from-the-closed-form-door-to-a-spline-carrier
kind: issue
title: the mint has no route to a spline carrier on an analytic chart, so such a face stays uncached under C4's exemption
status: closed
opened: 2026-10-02
priority: P1
closed: 2026-10-08
branch: pcert/projected-image
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

## Closed (branch `pcert/projected-image`, 2026-10-08)

The route is PR 4261's projected image. Both doors (`chart_pcurve`
and `chart_pcurve_over`, through `chart_image`) take a spline carrier
on an analytic chart to `Pcurve::Projected`: `ψ(C(t))` stored as the
carrier's net in the chart frame, its `atan2`-sector pieces and its
deck shift. Incidence is decided first (`net_incidence`): a net shown
off the chart refuses `CarrierOffChart`. `UncoveredClass::SplineCarrier`
is deleted, so `not_owed` no longer excuses the class, and the mint
mints or refuses such a face at the producer. The one remaining excusal
is a scalar without the fitted door (a dual), named by
`FittedLaneUnsupported`.
