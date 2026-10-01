---
id: chart-boundary-reads-an-on-locus-envelope-as-an-image-bound
kind: issue
title: chart_boundary widens a Fitted image's hull by an OnLocusHull envelope, which bounds the carrier's incidence, not the image
status: open
opened: 2026-10-01
---


Found by `pcert/general-circle-fitted-route` (2026-10-01), which made
it reachable at rest.

`topo::pcurves::chart_edge` (the `Pcurve::Fitted(_) | Pcurve::General(_)`
arm) describes a fitted image as its control hull widened by
`slack: cache.certificate().envelope`, and `chart_bound::MetredBound`'s
`hull` docs (`crates/topo/src/chart_bound.rs`, "a `Fitted`/`General`
edge's certificate allows the true image `slack` metres outside its
control hull") read that number as a bound on `|S(P(t)) − C(t)|`.

That holds for `EnvelopeStatement::MapResidualComposite` (a fitted image
on a spline chart). It does not hold for
`EnvelopeStatement::OnLocusHull`, the statement every fitted image on a
periodic ANALYTIC chart carries: there the envelope bounds the
CARRIER's distance from the chart surface, `sup |f_S(C(t))|`, and the
image's own displacement is certified only at the `CERT_SAMPLES`
schedule (`geom_brep::EnvelopeStatement::OnLocusHull`'s docs: "between
the samples it is bounded by nothing this statement says"). So the
metred hull a consumer cuts a carrier window to is not a certified
statement about where the true chart image lies between samples.

Reachable now: the oblique fillet trihedron's corner octants mint
`Fitted` rows on their sphere charts, and `chart_boundary` returns `Ok`
on four of them (`sweep`'s `m5_pr12_fix_pass::f4_...` exercises it).
Before, those faces stored no rows and `chart_boundary` refused at the
derivation.

Wanted: `chart_edge` reads `statement` before it reads `envelope`, and
refuses (or takes a bound that holds) for an `OnLocusHull` image —
e.g. the chart's Lipschitz bound times a certified sup of `P − ψ∘C`,
which the fitted lane does not compute today. The practical gap is
small (the image is interpolated at 8 nodes per CERT interval), but the
claim is a certificate's, and it is not one.
