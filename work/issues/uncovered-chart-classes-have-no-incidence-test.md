---
id: uncovered-chart-classes-have-no-incidence-test
kind: issue
title: Three uncovered chart_pcurve classes have no incidence test, so an off-chart carrier among them is excused as uncovered
status: open
opened: 2026-10-01
---


The residual of D36 (`pcert/d36-unsupported-carrier-split`). The ground
is `crates/geom-brep/src/pcurve_cache.rs`'s `chart_pcurve` (PCERT's and
PCTAIL's per `work.py territory`; filed here because that lane does not
write `work/pcert/`).

D36 split `UnsupportedCarrier` (uncovered: the carrier can lie on the
chart, no lane covers it — the only refusal `topo::pcurves::mint_faces`
excuses) from `CarrierOffChart` (a body defect, propagated). The sphere's
general-circle class is split by an incidence trilean
(`pcurve_sphere_chart_incident`, `sphere_general_circle`), and the
spiric's foreign charts by the mirror-torus and zero-offset arguments
(`spiric_off_own_chart`). Three classes have no such test and are
classified UNCOVERED wholesale, so a carrier in them that does NOT lie
on the chart is still excused by the mint and leaves its face uncached
and tier 3 clean:

- **torus, oblique circle** (`chart_pcurve`'s torus arm,
  `pcurve_torus_chart_meridian` failing): Villarceau circles lie on the
  torus, an arbitrary tilted circle does not. An incidence test is
  harder than the sphere's — the torus is quartic, so `|S∘C|` is not a
  three-coefficient identity — but `((|p|² + R² − r²)² − 4R²ρ²)` along
  `c + a cos t + b sin t` is a trigonometric polynomial of degree 4 in
  `t`, nine coefficients, each meterable.
- **cone, ellipse** (the cone arm's `Curve3::Ellipse`): a tilted plane
  section of the cone versus an ellipse off it. Incidence of a conic in
  a quadric is a degree-2 trigonometric identity, five coefficients.
- **analytic chart, spline carrier** (`chart_pcurve`'s top,
  `carrier_harmonic` answering `None`): no closed-form incidence test;
  would need the fitted lane's on-locus hull, which needs a mate.

Each would move its off-chart members to `CarrierOffChart` and make
the mint loud on them, as D36 did for the sphere.
