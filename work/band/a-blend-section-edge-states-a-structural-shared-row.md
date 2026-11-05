---
id: a-blend-section-edge-states-a-structural-shared-row
kind: unit
title: A blend's section edge states a structural shared row, certified on analytic charts at any contact order
status: open
opened: 2026-11-04
priority: P0
cost: H
parent: a-rounded-outline-is-a-closed-chain-of-mixed-links-the-blend-refuses
refs: [onpair-has-no-incidence-lane-on-a-spline-adjacent-pair]
---


Claimed by BAND (band-meridian lane, 2026-11-04). It sits on SELECT's ground (units 1–2 of `onpair-has-no-incidence-lane-on-a-spline-adjacent-pair`); SELECT has no orchestrator, and BAND claims only the analytic `SharedRow` slice.

**The ruling.** Two designers agreed, as a faithful reading of D2 (`docs/DESIGN.md` [edges]). Where a mixed outline's fillet bands meet G1, the edge is the rolling ball's section: the cylinder band's transverse circle, which is also the torus band's meridian. That curve is a row of both bands' charts and is known in advance. Its description is `Intersection { pair, selector: Structural(SharedRow) }`, not a witness `TangentIntersection`. No witness certificate could pass there: `κ_rel` vanishes exactly where the section meets the cap (`crates/geom-brep/src/tangent.rs`, doc of `tangent_certificate_lane`).

**The unit (geom-brep).**
- `Structural::SharedRow` certifies on analytic charts, at any contact order (`crates/geom-brep/src/certify.rs`, the `SharedRow` arms of `resolve_spec` and `run_checks`). The arm keeps the spline walls' domain-side rule and admits an analytic chart's row anywhere on it.
- Each wall's row image certifies through its own chart-image certificate.
- The row's contact class is read over the whole row (`crates/geom-brep/src/row_contact.rs`, `analytic_row_contact`). It is touching or crossing; a row that touches at a point and is crossing elsewhere refuses.
- A construction states the row through `geom_brep::analytic_shared_row`.

**Done when:** the stadium junction (cylinder against torus along the meridian) certifies as a touching shared row at `f64` and `Interval`, and the `band/closed-mixed-chain` lane states it in `attach_contact`.
