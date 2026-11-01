---
id: limb-two-has-three-break-grid-spellings
kind: issue
title: limb 2's round breaks are spelled three ways: cert_grid, analytic_limbs' range grid, chart_grid's closed-form arm
status: open
opened: 2026-10-14
priority: P4
cost: E
---


Found by the review of PR 5097 (`pcert/spline-pair-any-carrier`).

Limb 2's round breaks for a carrier are built in three places in
`crates/geom-brep/src/ssi/certify.rs`: `cert_grid` (a spline against an
analytic operand), `analytic_limbs`' `range_grid_points(t0, t1,
SSI_CERT_SPANS, &[])` arm (a closed form against an analytic operand)
and `chart_grid`'s closed-form arm (against a chart, deferring to the
image's knots). They agree in intent (an `SSI_CERT_SPANS` grid clear of
mandatory knots) and differ in what they defer to. One function taking
the carrier and the knots it must clear would make them one rule.
