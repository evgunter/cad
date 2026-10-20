---
id: a-circles-general-lane-is-written-only-on-the-sphere
kind: issue
title: A circle's general (projected) lane is written only on the sphere: on a cone, torus or cylinder a circle no closed form images refuses not yet, because the projected envelope has no certified circle-to-chart incidence sup there
status: open
opened: 2026-10-20
---


Filed by PCERT unit E1 (`pcert/lane-choices`), the D3 lane-choice
rewrite of the pcurve derivation.

## What

D3 (`docs/DESIGN.md`, [verdicts.consumers]): a lane choice takes the
closed form on Zero and the general lane on in band and definite, "and
every class a lane choice reads has one". For a circle carrier the
general lane is the projected image (`pcurve_cache/projected.rs`). Its
check 4 builds the envelope's `Incidence` term from a certified sup of
the circle's distance from the chart, and that sup is written for the
sphere only: `crate::sphere_circle::off_sphere_sup`, at
`projected.rs:1969` ("a circle's projected image is written for a
sphere"). The check-4 carrier gate at `projected.rs:2198` admits a
circle on a sphere only, for the same reason.

So on a cone, torus or cylinder, a circle whose class reading is in
band, or that is on the chart but none of its own circles (a cone
circle that is not a rim, a torus circle that is neither a parallel, a
meridian nor a Villarceau circle), has no general lane. Under D3's
interim rule it refuses lever-only "not yet" (`PcurveCheck::ChartClass`:
`Escalated` with the class reading's cause in band,
`PcurveCertifyError::GeneralLaneUnwritten` when definite). Before E1 the
definite case refused as `CarrierGrazesChart`, ending as a defect.

## What to build

A certified circle-to-chart incidence sup on each of the three charts,
so `run_projected_checks` admits the circle there. Two shapes:

- the closed-form sup of a harmonic carrier's distance from the chart:
  the cone's elevation (`crate::implicit::cone_elevation` is exact on
  the double cone), and the torus's `|√((ρ − R)² + h²) − r|`, bounded
  over `t` as `off_sphere_sup` bounds the sphere's;
- imaging the circle through its exact rational quadratic net, which
  the projected lane's net path (`FramedCarrier::Net`, `net_incidence`)
  already certifies on every analytic chart, with check 4's carrier gate
  comparing that net with the circle's own.

Rows: a cone circle that is not a rim, a torus crest circle, and an
in-band rim, each minting a projected image with its envelope bounding
the sampled residual, at `f64` and at `Interval`.
