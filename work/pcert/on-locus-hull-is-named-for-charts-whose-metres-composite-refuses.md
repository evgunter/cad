---
id: on-locus-hull-is-named-for-charts-whose-metres-composite-refuses
kind: issue
title: fitted_lane names OnLocusHull for the cone and torus, while ssi::certify::composite_form refuses their metres conversion
status: closed
opened: 2026-10-07
closed: 2026-10-08
branch: pcert/projected-image
---


Found by both spline-carrier designers (PR 4261, fork-log row 85).

The `fitted_lane` doc and C4 name `OnLocusHull` as the between-samples envelope for a fitted image over a rung-3 carrier on a periodic analytic chart, the cone and torus among them. But `ssi::certify::composite_form` refuses the metres conversion on the cone and torus: it needs a root, and certification arithmetic takes none (C2).

This is consistent today only because no rung-3 arm mints on those charts. A row that did would get no between-samples bound.

PR 4261's route retires `OnLocusHull` on analytic charts and states incidence with a per-span lever, which resolves this. If that route is not taken, the text must say which charts the statement covers.

## Closed (branch `pcert/projected-image`, 2026-10-08)

PR 4261's route was taken. `EnvelopeStatement::OnLocusHull` is
deleted, and a rung-3 carrier on an analytic chart stores its projected
image. Its incidence term is `implicit_composite`'s span bound read
through the chart's metres conversion: a constant on the plane, sphere
and cylinder, and a per-span lever on the cone
(`ρ_min cos α + (σz)_min sin α`) and the torus (`r((ρ_min+R)²−r²)`).
The lever takes no root: `ρ_min` comes from the span box's distance to
the axis (`projected::rho_range`) using Interval ops only.
`run_fitted_checks` now refuses a fitted image on an analytic chart
(`ImageMismatch { image: Fitted, .. }`).
