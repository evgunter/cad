---
id: plane-chart-polygon-erases-the-chart-pcurve-refusal
kind: issue
title: chart_region's plane-chart polygon erases chart_pcurve's typed refusal behind one string
status: open
opened: 2026-10-01
---


Found by the D36 sweep (`pcert/d36-unsupported-carrier-split`).

`crates/topo/src/chart_region.rs`, the plane-chart arm of the loop
polygon extraction (`geom_brep::chart_pcurve(..).map_err(|_| refuse("no
closed-form chart image for this carrier kind"))`, ~line 2475), erases
`chart_pcurve`'s typed refusal. On a plane chart that refusal is now one
of: `UnsupportedCarrier { carrier: "Nurbs", .. }` (a spline carrier —
what the string says), `CarrierOffChart` (not reachable on a plane
today: no plane arm refuses off-chart, but the type does not say so),
or `Escalated` (a classification in the sliver band, which the string
reports as a missing carrier kind). The escalation is the live loss:
it is reported as a kind the kernel lacks rather than as a margin too
close to call, and its `Indeterminate` diagnostic is dropped.

Wanted: map `Escalated` to the region's own escalation arm and keep
the carrier refusal's payload (or nest the error), as `chord_join.rs`'s
`chart_pcurve` call already separates the escalation.
